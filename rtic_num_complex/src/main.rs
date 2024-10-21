#![no_std]
#![no_main]

/*
Examples I used:
https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/rtic-i2s-audio-in-out.rs
*/

use defmt_brtt as _;

use panic_probe as _;

mod filter;
mod fft;
mod vad;
mod vad_utils;

const BUFFER_SIZE : usize = 640;
const SAMPLING_RATE : usize = 8_000;

const FRAME_DURATION :f32 = 0.01;
const FRAME_LENGTH :usize = (FRAME_DURATION * SAMPLING_RATE as f32) as usize + 48;
const ACTUAL_FRAME_LENGTH :usize = (FRAME_DURATION * SAMPLING_RATE as f32) as usize;
const NUMBER_OF_FRAMES :usize = BUFFER_SIZE / ACTUAL_FRAME_LENGTH;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [EXTI0, EXTI1, EXTI2])]
mod app {

    const BUFFER_SIZE: usize = 640;
    const SAMPLING_RATE : usize = 8_000;

    use core::f32::consts::PI;

    use num_complex::ComplexFloat;

    use embedded_graphics::{
        prelude::Point,
        prelude::Primitive,
        primitives::PrimitiveStyle,
        primitives::Line,
        prelude::RgbColor,
        Drawable,
        draw_target::DrawTarget,
        pixelcolor::Rgb565
    };

    use stm32f4xx_hal::{
        gpio::alt::i2s2::{Ck,Mck,Sd,Ws},
        gpio::NoPin,
        gpio::Pin,
        gpio,
        pac::*,
        spi::Spi,
        spi::{Phase, Polarity,Mode},
        i2s::*,prelude::*,block,timer::SysDelay
    };

    use st7735_lcd::ST7735;

    use embedded_hal_bus::spi::ExclusiveDevice;

    use stm32_i2s_v12x::{
        driver::*,
        transfer::{I2sTransfer, I2sTransferConfig}
    };

    use crate::vad;

    type I2STransfer =  I2sTransfer<I2s<SPI2>, Master, Receive, Philips, Data16Channel32>;
    
    type SpiDisplay = ST7735<ExclusiveDevice<Spi<SPI1>,Pin<'D', 14, gpio::Output>, SysDelay>, Pin<'D', 15, gpio::Output>, Pin<'F', 12, gpio::Output>>;

    #[shared]
    struct Shared{
        #[lock_free]
        i2s_transfer : I2STransfer,
        #[lock_free]
        display : SpiDisplay
    }

    #[local]
    struct Local {
    }

    #[init(local = [])]
    fn init(cx: init::Context) ->  (Shared, Local, init::Monotonics)
    {
        
        let cp = cx.core;
        let dp = cx.device;

        let rcc = dp.RCC.constrain();

        let clocks = rcc.cfgr
                .use_hse(8.MHz())
                .sysclk(100.MHz())
                .i2s_clk(61440.kHz())
                .freeze();
        
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();
        let gpiod = dp.GPIOD.split();
        let gpiof = dp.GPIOF.split();

        let ck_pin = gpiob.pb13.into_alternate();
        let ck = Ck::PB13(ck_pin);
    
        //master clock
        let mck_pin = NoPin::new();
        let mck = Mck::None(mck_pin);
  
        //serial data
        let sd_pin = gpiob.pb15.into_alternate();
        let sd = Sd::PB15(sd_pin);
   
        //word select
        let ws_pin = gpiob.pb9.into_alternate();
        let ws = Ws::PB9(ws_pin);
    
        let peripheral = I2s::new(dp.SPI2, (ws,ck,mck,sd) , &clocks);
         
        let transfer_config = I2sTransferConfig::new_master()
                                            .receive()
                                            .standard(Philips)
                                            .data_format(Data16Channel32)
                                            .request_frequency(SAMPLING_RATE as u32);

        let transfer =
            I2sTransfer::new(peripheral, transfer_config);

        let delay = cp.SYST.delay(&clocks);
        
        let rst = gpiof.pf12.into_push_pull_output();
        let dc = gpiod.pd15.into_push_pull_output();

        let cs = gpiod.pd14.into_push_pull_output();
        
        let mut display_delay = dp.TIM5.delay_us(&clocks);
    
        let spi = Spi::new(
            dp.SPI1,
            (gpioa.pa5, gpioa.pa6, gpioa.pa7),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            16.MHz(),
            &clocks,
        );
     
        let spi_device=
            ExclusiveDevice::new(spi, cs, delay).unwrap();        
     
        let mut display  = 
            ST7735::new(spi_device, dc, rst, true, false, 128, 128);
     
 
        let _ = display.init(&mut display_delay);
     
        let _ = display.set_orientation(&st7735_lcd::Orientation::LandscapeSwapped);
     
        let _ = display.clear(Rgb565::BLACK);

        (
            Shared {
                i2s_transfer: transfer,
                display : display
            },
            Local {
                
            },
            init::Monotonics(),
        )
    }


   #[idle(shared = [display,i2s_transfer], local = [])]
    fn idle(cx: idle::Context) -> ! {

        let f = 2511.0;

        let mut desired = [0.0; BUFFER_SIZE];

        /*
        The following code used to generate the sine wave has been inspired by:
        https://uk.mathworks.com/help/signal/ug/amplitude-estimation-and-zero-padding.html
        */
        let mut m = 0.001;
        let mut i = 0;
        while m < (BUFFER_SIZE as f32 / 1000.0){
            let sin = 2.0*PI*f*m;
            desired[i] = 10.0* sin.sin();
            i += 1;
            m += 0.001;
        }

        let transfer = cx.shared.i2s_transfer;
        let display = cx.shared.display;
       

        let mut line = Line::new(Point::new( 0,0), Point::new( 0,0) );
        let mut last_point = Point::new(0,0);
    
        let refresh_time = 2;
        let mut iteration_count = 0;

        loop{

            let mut frames = [0.0; BUFFER_SIZE];
    
            for s in 0..BUFFER_SIZE{
                if let Ok((l,_)) = block!(transfer.read()){
                    frames[s] = (l >> 6) as f32;
                }
            }

         
            let data = vad::vad(frames);

            let filter_results = if data.1 { 
                                            crate::filter::lms(desired, data.0, 0.00000001)
                                        } else{  
                                          data.0
                                        };


            if iteration_count == refresh_time {
                let _= display.clear(Rgb565::BLACK);
                iteration_count = 0;
    
            }

            for i in 0..BUFFER_SIZE {

                let idx = if i > 1 {
                    i-1 
                } else {
                    i
                };
    
                let mut element_start = if i == 0{
                    filter_results[i] as i32
                
                } else {
                    0
                };
    
                let mut element_end = if i == 0{
                    filter_results[i+1] as i32
                } else {
                    filter_results[i] as i32
                };

                let increase = 10;
                element_start *= increase;
                element_end *= increase;
    
                 let mut offset = 0;
                if iteration_count > 0 && iteration_count <= refresh_time{
                    offset = 34 * iteration_count;
                } 
                
                if i == 0 {
                    line.start  = Point::new( (idx as i32) + offset, 64 - element_start);
                    line.end = Point::new( (idx as i32) + 4 + offset, 64 - element_end);
                }
                else {
                    line.start  = last_point;
                    line.end = Point::new( (idx as i32) * 4 + 10 + offset, 64 - element_end);
                }

                last_point = line.end;

                line.into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1)).draw(display).unwrap();
            }
            iteration_count += 1;
        }
    }
}

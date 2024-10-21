use num_complex::ComplexFloat;
use complex_nums_wrapper::ComplexNum;
use crate::{ACTUAL_FRAME_LENGTH, BUFFER_SIZE, FRAME_LENGTH, NUMBER_OF_FRAMES, SAMPLING_RATE};


/*
This function calculates the complex magnitude for each array in the 2d array input.

The input to sfm is always the magnitudes of the fft see
https://www.researchgate.net/publication/224078693_Note_on_measures_for_spectral_flatness
*/
pub fn frames_mag(array:[[ComplexNum;FRAME_LENGTH];NUMBER_OF_FRAMES]) -> [[f32;FRAME_LENGTH];NUMBER_OF_FRAMES]{
    let mut out = [[0.0; FRAME_LENGTH];NUMBER_OF_FRAMES];

    for i in 0..NUMBER_OF_FRAMES{
         out[i] = mag(array[i]);
    }

    out
}

/*
This function calculates the magnitude of a complex number. And is does so for every element in a complex array.
*/
pub fn mag(array:[ComplexNum;FRAME_LENGTH]) -> [f32;FRAME_LENGTH]{
    let mut out= [0.0;FRAME_LENGTH];

    for i in 0..FRAME_LENGTH{
        let temp = array[i].real * array[i].real + array[i].im * array[i].im;
       
        out[i] = temp.sqrt();
    }

    out
}

/*
As a reference for the following calculation, I used the following sources:

https://stackoverflow.com/questions/17390677/how-can-i-get-dft-fft-output-frequencies-in-hertz

https://www.youtube.com/watch?v=3aOaUv3s8RY&ab_channel=MarkNewman
*/
pub fn get_frequency(index: usize)-> i32{
    ((index as i32) * (SAMPLING_RATE as i32))/(BUFFER_SIZE as i32)
}

/*
The order of highest energies in audio samples is structured as follows:
1. speech
2. unvoiced (e.g. instruments)
3. silence
(see the links below
https://superkogito.github.io/blog/2020/02/09/naive_vad.html
https://www.clear.rice.edu/elec532/PROJECTS00/vocode/uv/uvdet.html
)
*/
pub fn short_term_energy(frames:[[f32; FRAME_LENGTH]; NUMBER_OF_FRAMES])-> [f32; NUMBER_OF_FRAMES]{
    let mut out = [0.0;NUMBER_OF_FRAMES];

    for i in 0..NUMBER_OF_FRAMES {
        let mut temp = 0.0;

        for j in 0..FRAME_LENGTH{
            temp += frames[i][j] * frames[i][j];
        }

        out[i] = temp;
    }
    out
}

/*
A reference for the framing implementation provided here is:

https://brianmcfee.net/dstbook-site/content/ch09-stft/Framing.html
*/
pub fn get_frames_with_padding(signal:[f32; BUFFER_SIZE])-> [[f32; FRAME_LENGTH]; NUMBER_OF_FRAMES]{

    let mut array_2d = [[0.0; FRAME_LENGTH];NUMBER_OF_FRAMES];

    for i in 0..NUMBER_OF_FRAMES{
        for j in 0..FRAME_LENGTH{
            array_2d[i][j] = if ACTUAL_FRAME_LENGTH < j{
                                0.0
                            } else{
                                signal[j + i*ACTUAL_FRAME_LENGTH]
                            };
        }
    }

    array_2d
}

/*
This function calculates the spectral flatness measure (SFM) for a 2D array.

Madhu, Nilesh. (2009). Note on measures for spectral flatness. Electronics Letters. 45. 1195 - 1196. 10.1049/el.2009.1977. 
*/
pub fn sfm_overall(magnitudes: [[f32; FRAME_LENGTH];NUMBER_OF_FRAMES]) -> [f32; NUMBER_OF_FRAMES]{
    let mut out = [0.0;NUMBER_OF_FRAMES];

    for i in 0..NUMBER_OF_FRAMES{
        out[i] = sfm(magnitudes[i]);
    }
    out
}

/*
The SFM is a ratio of geometric mean to arithmetic mean of the magnitude spectrum from the FFT.
So, calculate the magnitude and then input it into SFM.

Madhu, Nilesh. (2009). Note on measures for spectral flatness. Electronics Letters. 45. 1195 - 1196. 10.1049/el.2009.1977. 

Defnition of the magnitude spectrum:
https://www.sciencedirect.com/topics/computer-science/magnitude-spectrum
*/
pub fn sfm(magnitudes_of_fft:[f32; FRAME_LENGTH]) -> f32{
    let mut out = [0.0;FRAME_LENGTH];

    for k in 0..FRAME_LENGTH{
        let mut geometric = 1.0;
        let mut arithmetic = 0.0;

        for i in 0..k+1{
            geometric *= magnitudes_of_fft[i];
            arithmetic += magnitudes_of_fft[i];
        }

        let idx = k+1;
        
        let geometric_mean = geometric.powf(1.0/idx as f32);
        let arithmetic_mean = arithmetic * (1.0/idx as f32);
        let div = geometric_mean/arithmetic_mean;

       out[k] = 10.0 * div.log10(); //tradintionaly: just out[k] = div;
      
    }

    out[FRAME_LENGTH-1]
}

pub fn find_min_overall(array:[f32;NUMBER_OF_FRAMES]) -> (f32,usize){
    
    let mut min = core::f32::MAX;
    let mut idx = 0;

    for h in 0..NUMBER_OF_FRAMES{
        let element = array[h];
       if element < min {
                min = element;
                idx = h;
        }
    }

   (min, idx)
}

pub fn find_max_2d(array:[[f32;FRAME_LENGTH];NUMBER_OF_FRAMES])-> (f32,usize,usize){

    let mut max = core::f32::MIN;
    let mut outer_idx = 0;
    let mut inner_idx= 0;

    for t in 0..NUMBER_OF_FRAMES {
        for j in 0..FRAME_LENGTH{
            let element = array[t][j];
            if  element > max {
                max =  element;
                outer_idx = t;
                inner_idx = j;
            }
        }
    }
    (max,outer_idx,inner_idx)
}

pub fn find_min_2d(array:[[f32;FRAME_LENGTH];NUMBER_OF_FRAMES])-> (f32,usize,usize){

    let mut min = core::f32::MAX;
    let mut outer_idx = 0;
    let mut inner_idx= 0;

    for t in 0..NUMBER_OF_FRAMES {
        for j in 0..FRAME_LENGTH{
            let element = array[t][j];
            if  element < min {
                min =  element;
                outer_idx = t;
                inner_idx = j;
            }
        }
    }
    (min,outer_idx,inner_idx)
}


use num_complex::ComplexFloat;
use crate::my_cmsis_complex::MyComplex;
use crate::{BUFFER_SIZE,NUMBER_OF_FRAMES,FRAME_LENGTH,ACTUAL_FRAME_LENGTH};

use crate::vad_utils::*;
use crate::fft;

/*
The following code has been inspired by:
M. H. Moattar and M. M. Homayounpour, "A simple but efficient real-time Voice Activity Detection algorithm," 2009 17th European Signal Processing Conference, Glasgow, UK, 2009, pp. 2549-2553.
*/
pub fn vad(signal : [f32;BUFFER_SIZE]) -> ([f32;BUFFER_SIZE], bool){

    let mut out = [0.0;BUFFER_SIZE];

    let mut signal_contains_speech = false;

    let frames = get_frames_with_padding(signal);

    let mut silence_count = 0;
    let mut speech_count = 0;

    let energy_prim_threshold = 40.0;
    let threshold_frequency = 185;
    let threshold_sfm = 5.0;

    let energy_frame = short_term_energy(frames);

    let mut fft_input = [[MyComplex::new(0.0, 0.0);FRAME_LENGTH];NUMBER_OF_FRAMES];
    for i in 0..NUMBER_OF_FRAMES{
        for j in 0..FRAME_LENGTH{
            fft_input[i][j] = MyComplex::new(frames[i][j] as f32, 0.0);
        }
    }
    let fft_frame = fft::frames_fft(fft_input);
    let fft_magnitudes = frames_mag(fft_frame);
        
    let frequency_input_f_i = find_max_2d(fft_magnitudes);
    let f_i = get_frequency(frequency_input_f_i.1 * ACTUAL_FRAME_LENGTH + frequency_input_f_i.2);

    let sfm = sfm_overall(fft_magnitudes);

    let mut min_e =  find_min_overall(energy_frame).0;
        
    let frequency_input_min_f = find_min_2d(fft_magnitudes);
    let min_f = get_frequency((frequency_input_min_f.1) * ACTUAL_FRAME_LENGTH + (frequency_input_min_f.2));
        
    let min_sfm = find_min_overall(sfm).0;

    let mut threshold_e = energy_prim_threshold * min_e.log10();

    for i in 0..NUMBER_OF_FRAMES{

        let e_i = energy_frame[i];

        let sfm_i = sfm[i];
            
        let mut counter = 0;
            
        if (e_i - min_e) >= threshold_e{
            counter += 1;
        }

        if (f_i - min_f) >= threshold_frequency{
            counter += 1;
        }
       
        if (sfm_i - min_sfm) >= threshold_sfm{
            counter += 1;
        }
    
        if counter > 1{
            //speech detected
            speech_count += 1;
                
        }
        else {
            //silence detected
            silence_count += 1;

            min_e = ((silence_count as f32 * min_e) + e_i)/(silence_count as f32 + 1.0);
            threshold_e = energy_prim_threshold * min_e.log10();
                
        }
    }
    
    if silence_count != 0 && speech_count >= NUMBER_OF_FRAMES-2{
        out = signal;
        signal_contains_speech = true;
    }
    
    (out, signal_contains_speech)
}

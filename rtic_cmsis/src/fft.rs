use core::f32::consts::PI;
use num_complex::ComplexFloat;

use crate::my_cmsis_complex::MyComplex;

use crate::{FRAME_LENGTH, NUMBER_OF_FRAMES};

pub fn log2(number : usize) -> usize{
    let mut out = 0;
    let mut n = number;
    while n > 0 {
        n = n >> 1;
        out += 1;
    }
    out-1
}

/*
This implementation of the fft has been inspired by:
https://github.com/JanMarcelKezmann/Fast-Fourier-Transformation-with-Python-and-Numpy/blob/master/Fast_Fourier_Transform.ipynb
*/
pub fn fft(y:[MyComplex; FRAME_LENGTH]) -> [MyComplex; FRAME_LENGTH]{

    let n = log2(FRAME_LENGTH) as u32;

    let mut a = mybitrev(y);

    for m in 1..n+1{
        for z in 0..(1 << m-1){
            let j = z as f32;
            let co = -2.0 * PI * j/ (1 << m) as f32;
            let si = -2.0 * PI * j/ (1 << m) as f32;
            let e = MyComplex::new(co.cos(), si.sin());
            
            let mut r = 0;
            while r < FRAME_LENGTH{
                let temp = a[r + z + (1 << m-1)]*e;
                a[r + z + (1 << m-1)] = a[r + z] - temp;
                a[r+z] = a[r + z] + temp;

                r += 1 << m;
            }       
        }
    }

    a
}

pub fn frames_fft(input_frames : [[MyComplex; FRAME_LENGTH];NUMBER_OF_FRAMES]) ->[[MyComplex; FRAME_LENGTH];NUMBER_OF_FRAMES]{
    let mut out = [[MyComplex::new(0.0,0.0);FRAME_LENGTH];NUMBER_OF_FRAMES];

    for u in 0..NUMBER_OF_FRAMES{
        out[u] = fft(input_frames[u]);
    }

    out
}

/*
The following bit reversal implementation only works for the input arrays, that has a length of the power of two.
*/
pub fn mybitrev(array:[MyComplex; FRAME_LENGTH]) -> [MyComplex;FRAME_LENGTH]{

    let mut len2 = 0;
    let mut len_in_bits = 0;
    while len2 < FRAME_LENGTH{
        len2 = 1 << len_in_bits;
        len_in_bits += 1;
    }
    len_in_bits -= 2;   
    /*
    Minus two, because len_in_bits is bigger than len and
    bits start at 2^0=1 which should not be counted as a shift.
    */

    let mut a = [MyComplex::new(0.0,0.0);FRAME_LENGTH];

    for i in 0..FRAME_LENGTH{

        let mut new_idx = 0;

        /*
        Iterate over each index, but this implementation already checks for symmetry so it only has
        to iterate over the half of the array. And since the loop ends at k < len_in_bits/2 
        it needs to add a one to get the iteration with k = len_in_bits/2.
        */
        for k in 0..(len_in_bits/2)+1{

            let other_side = len_in_bits - k;
            
            if other_side == k {
                if (1 << other_side) & i == (1 << other_side){
                    new_idx += 1 << k;
                }
            }
            else {

                if (1<<k) & i == (1<<k){

                    if (1<<other_side) & i == (1<<other_side){
                        //The number i is symmetric, so new_idx should also be symmetric
                        new_idx += 1 << other_side;
                        new_idx += 1 << k;
                    }
                    else {
                        new_idx += 1 << other_side;
                        //Unsetting 1 in i is not necessary since it is unset by default, see initialization
                    }
                }
                else {
                    if (1<<other_side) & i == (1 << other_side){
                        new_idx += 1 << k;
                        //Here you also don't need to unset the bit at 1 << other_side; unset by default
                    }
                }
            
            }
        }
        
        //swap
       a[i] = array[new_idx];
       a[new_idx] = array[i];
    }
    a
}

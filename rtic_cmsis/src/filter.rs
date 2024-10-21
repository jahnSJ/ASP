use crate::BUFFER_SIZE;

/*
The following code has been inspired by:
Mekhfioui, Mohcin & Elgouri, Rachid & Satif, Amal & Moumouh, Meryem & Laâmari, HLOU. (2020). Implementation Of Least Mean Square Algorithm Using Arduino & Simulink. International Journal of Scientific & Technology Research. 9. 664-667.
*/
pub fn lms(desired: [f32;BUFFER_SIZE], input_x:[f32;BUFFER_SIZE], mu : f32) -> [f32;BUFFER_SIZE] {
    let mut h = [0.0;BUFFER_SIZE];
    let mut e = [0.0;BUFFER_SIZE];

    for i in 1..BUFFER_SIZE{
       
        let x = h[i-1] * input_x[i];
        e[i] = desired[i] - x;
        h[i] = h[i-1] + 2.0 * mu * input_x[i] * e[i];
    }

    e
}

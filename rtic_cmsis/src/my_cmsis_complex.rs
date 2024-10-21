use cmsis_dsp::basic::*;

use core::{
	ops::Mul,
	ops::Add,
	ops::Sub,
	ops::Div
};

/*
The following code has been inspired by:
https://docs.rs/num-complex/latest/src/num_complex/lib.rs.html#811
*/
#[derive(Copy,Clone)]
pub struct MyComplex{
    pub real : f32,
    pub imaginaer : f32
}


impl MyComplex{

    pub fn new(re:f32, im:f32)-> MyComplex{
        MyComplex{
            real : re,
            imaginaer : im
        }
    }
}


impl Add<MyComplex> for MyComplex{

    type Output = Self;

    fn add(self, num2: Self) -> Self::Output{
    	let num_one = [self.real,self.imaginaer];
    	let num_two = [num2.real,num2.imaginaer];
       	let mut result = [0.0;2];
       	add_f32(&num_one, &num_two, &mut result);
       	
        Self::Output::new(result[0],result[1])
    }
}

impl Sub<MyComplex> for MyComplex{

    type Output = Self;

    fn sub(self, num2: Self) -> Self::Output{
        let num_one = [self.real,self.imaginaer];
    	let num_two = [-num2.real,-num2.imaginaer];
       	let mut result = [0.0;2];
       	add_f32(&num_one, &num_two, &mut result);
       	
        Self::Output::new(result[0],result[1])
    }
}

impl Div<MyComplex> for MyComplex{

    type Output = Self;
    /*
        from: https://www.studysmarter.de/schule/mathe/algebra/komplexe-zahlen/ 
    */
    fn div(self, nenner: Self) -> Self::Output{

        let mut real = [0.0];
        let mut im = [0.0];

        	let multiply_one = [nenner.real, nenner.imaginaer, self.real, self.imaginaer, self.imaginaer, self.real];
        	let multiply_two = [nenner.real, nenner.imaginaer, nenner.real, nenner.imaginaer, nenner.real, nenner.imaginaer];
        	let mut result = [0.0;6];
        	multiply_f32(&multiply_one, &multiply_two, &mut result);
        	
        	let divident_part_one = [result[0]];
        	let divident_part_two = [result[1]];
        	let mut divident_as_num = [0.0];
        	add_f32(&divident_part_one, &divident_part_two, &mut divident_as_num);
        	
        	let divident = [1.0/divident_as_num[0]];//there is sadly no division for f32 in cmsis, but how does it get executed??
        	
        	let real_part_one = [result[2]];
        	let real_part_two = [result[3]];
        	let mut temp_result = [0.0];
        	add_f32(&real_part_one, &real_part_two, &mut temp_result);
        	
        	multiply_f32(&divident, &temp_result, &mut real);
        	
        	let im_part_one = [result[4]];
        	let im_part_two = [-result[5]];
        	let mut temp_result_2 = [0.0];
        	add_f32(&im_part_one, &im_part_two, &mut temp_result_2);

        	multiply_f32(&divident, &temp_result_2, &mut im);
       
        Self::Output::new(real[0],im[0])
    }
}

impl Mul<MyComplex> for MyComplex{

    type Output = Self;

    fn mul(self, num2: Self) -> Self::Output{
    	
    	let num_one = [self.real, self.imaginaer, self.real, self.imaginaer];
    	let num_two = [num2.real, num2.imaginaer, num2.imaginaer, num2.real];
       	let mut result = [0.0;4];
       	multiply_f32(&num_one, &num_two, &mut result);
       	
       	let real_part_1 = [result[0]];
       	let real_part_2 = [-result[1]];
       	let mut real = [0.0];
       	add_f32(&real_part_1, &real_part_2, &mut real);
       	
       	let im_part_1 = [result[2]];
       	let im_part_2 = [result[3]];
       	let mut im = [0.0]; 
       	add_f32(&im_part_1, &im_part_2, &mut im);

        Self::Output::new(real[0],im[0])        
    }
}

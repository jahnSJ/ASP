#![no_std]

use core::{
	ops::Mul,
	ops::Add,
	ops::Sub,
	ops::Div
};

use complex_nums::{add, sub, multiply, div, init, MyComplex};

#[derive(Copy,Clone)]
pub struct ComplexNum{
   pub real: f32,
   pub im : f32,
}


impl ComplexNum {
    pub fn new(real:f32, im:f32)-> ComplexNum{
        ComplexNum { 
            real, 
            im
        }
    }

    pub fn initilization(){
        unsafe{
            init();
        }
    }

}
impl Add<ComplexNum> for ComplexNum{

    type Output = Self;

    fn add(self, rs_op2: Self) -> Self::Output{
        let mut output: ComplexNum = ComplexNum::new(0.0,0.0);
        unsafe{
            let c_op1 = MyComplex{
                real : self.real, 
                imaginaer : self.im
            };

            let c_op2 = MyComplex{
                real : rs_op2.real, 
                imaginaer : rs_op2.im
            };

            let res = add(c_op1, c_op2);
            output.real = res.real;
            output.im = res.imaginaer;
        }
        Self::Output::new(output.real,output.im)
    }
}

impl Sub<ComplexNum> for ComplexNum{

    type Output = Self;
    
    fn sub(self, rs_op2: Self)-> ComplexNum{
        let mut output: ComplexNum = ComplexNum::new(0.0,0.0);
        unsafe{
            let c_op1 = MyComplex{
                real : self.real, 
                imaginaer : self.im
            };

            let c_op2 = MyComplex{
                real : rs_op2.real, 
                imaginaer : rs_op2.im
            };

            let res = sub(c_op1, c_op2);
            output.real = res.real;
            output.im = res.imaginaer;
        }
        Self::Output::new(output.real,output.im)
    }

}

impl Mul<ComplexNum> for ComplexNum{

    type Output = Self;

    fn mul(self, rs_op2:Self) -> Self::Output{
        let mut output: ComplexNum = ComplexNum::new(0.0,0.0);
        unsafe{
            let c_op1 = MyComplex{
                real : self.real, 
                imaginaer : self.im
            };

            let c_op2 = MyComplex{
                real : rs_op2.real, 
                imaginaer : rs_op2.im
            };

            let res = multiply(c_op1, c_op2);
            output.real = res.real;
            output.im = res.imaginaer;
        }
        Self::Output::new(output.real,output.im)
    }
}

impl Div<ComplexNum> for ComplexNum{

    type Output = Self;
    
    fn div(self, rs_op2: Self)  -> Self::Output{
        let mut output: ComplexNum = ComplexNum::new(0.0,0.0);
        unsafe{
            let c_op1 = MyComplex{
                real : self.real, 
                imaginaer : self.im
            };

            let c_op2 = MyComplex{
                real : rs_op2.real, 
                imaginaer : rs_op2.im
            };

            let res = div(c_op1, c_op2);
            output.real = res.real;
            output.im = res.imaginaer;
        }
        Self::Output::new(output.real,output.im)
    }
}

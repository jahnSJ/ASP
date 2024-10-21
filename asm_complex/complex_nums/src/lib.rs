#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


/*
#[test]
fn test() {
    let mut one = 0;
    let mut two = 0;
    unsafe{
        let op1 = MyComplex{
            real : 3,
            imaginaer : 4
        };
        let op2 = MyComplex{
            real : 6,
            imaginaer : 6
        };
        let res = add(op1, op2);
        one = res.real;
        two = res.imaginaer;
    }
    assert!(one == 9 && two == 10);
}
 */

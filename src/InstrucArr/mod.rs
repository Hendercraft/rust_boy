#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::Hardware;
use crate::Hardware::Instruct;
use Hardware::Op as Op;


mod InstrucFn;

pub fn createOperations() -> Vec<Hardware::Instruct>{
    let mut out = Vec::new();
    for i in 0..256{
        out.push(Hardware::Instruct {
            n : i,
            name : String::from("NOP"),
            desc : String::from("Aussi inutile que les cours de GE00"),
            argc : 1,
            tics : 4,
            exec : Op::no(InstrucFn::nop),
        })
    }

    out[2] = Hardware::Instruct::build_instruct(2, String::from("LD BC,d16"), String::from("Load the d16 given in the operhand in BC"), 2, 12, Op::no(InstrucFn::nop));
    return out;
}

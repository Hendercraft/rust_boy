#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::Hardware;
use Hardware::Instruct as Instruct;
use Hardware::Op as Op;



mod InstrucFn;

pub fn createOperations() -> Vec<Instruct>{
    let mut out = Vec::new();
    for i in 0..256{
        out.push(Instruct {
            n : i,
            name : String::from("NOP"),
            desc : String::from("Aussi inutile que les cours de GE00"),
            argc : 0,
            ticks: 4,
            exec : Op::no(InstrucFn::nop),
        })
    }

    out[0x01] = Instruct::build_instruct(1, String::from("LD BC,d16"), String::from("Load the d16 given in the operand in BC"), 2, 12, Op::u16toCpu(InstrucFn::ld_bc_u16));
    out[0x02] = Instruct::build_instruct(2, String::from("LD (BC) A"), String::from("Load A to the ram adress specified in BC "), 0, 8, Op::ram(InstrucFn::ld_bcp_a));
    out[0x03] = Instruct::build_instruct(3, String::from("INC BC"), String::from("Increase BC "), 0, 8, Op::no(InstrucFn::inc_bc));
    out[0x04] = Instruct::build_instruct(4, String::from("INC B"), String::from("Increase B, manipulate Z,H flag, set N to 0"), 0, 4, Op::no(InstrucFn::inc_b));
    out[0x05] = Instruct::build_instruct(5, String::from("DEC B"), String::from("Decrease B, manipulate Z,H flag, set N to 1"), 0, 4, Op::no(InstrucFn::dec_b));
    out[0x06] = Instruct::build_instruct(6, String::from("LD B d8"), String::from("Load the d8 given in the operand in B"), 1, 8, Op::u8toCpu(InstrucFn::ld_b_u8));
    out[0xD9] = Instruct::build_instruct(0xD9, String::from("RETI"), String::from("Load the sp back to pc (return from interrupt)"), 0, 16, Op::ram(InstrucFn::reti));
    return out;

}

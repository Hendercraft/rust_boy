#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::Hardware;
use crate::Hardware::Cpu as Cpu;

pub fn nop(cpu : &mut Cpu){}

pub fn ld_bc_d16(cpu : &mut Cpu, high : u8, low : u8){

}
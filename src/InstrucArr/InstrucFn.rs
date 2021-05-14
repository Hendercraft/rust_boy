#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::Hardware;
use crate::Hardware::Cpu as Cpu;

//0x01
pub fn nop(cpu : &mut Cpu){}

//x02
pub fn ld_bc_d16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_b(h);
    cpu.set_c(l);
}
//x03
pub fn ld_bcp_a(cpu : &mut Cpu, ram : &mut Vec<u8>){
    ram[cpu.get_bc() as usize] = cpu.get_a();
}
//x04 //A VERIFIER PAS SUR DUTOUT
pub fn inc_bc(cpu : &mut Cpu){
    cpu.set_bc(cpu.get_bc() + 0b1);

}
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::Hardware;
use crate::Hardware::Cpu as Cpu;
use crate::Hardware::Flag::*;

//0x00
pub fn nop(cpu : &mut Cpu){}

//x01
pub fn ld_bc_d16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_b(h);
    cpu.set_c(l);
}
//x02
pub fn ld_bcp_a(cpu : &mut Cpu, ram : &mut Vec<u8>){
    ram[cpu.get_bc() as usize] = cpu.get_a();
}
//x03
pub fn inc_bc(cpu : &mut Cpu){
    cpu.set_bc(cpu.get_bc() + 0b1);
}
//x04
pub fn inc_b(cpu :&mut Cpu){
    if cpu.get_b() == 0x0f { //Half carry falg
        cpu.set_flag(H);
    }else{cpu.clear_flag(H)}
    cpu.set_b(cpu.get_b() + 0b1);

    if cpu.get_b() == 0 {cpu.set_flag(Z);} else { cpu.clear_flag(Z) } //zero flag

    cpu.clear_flag(N);
}


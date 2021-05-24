#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::Hardware;
use crate::Hardware::Cpu as Cpu;
use crate::Hardware::Flag::*;

//0x00
pub fn nop(cpu : &mut Cpu){}

//0x01
pub fn ld_bc_d16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_b(h);
    cpu.set_c(l);
}
//0x02
pub fn ld_bcp_a(cpu : &mut Cpu, ram : &mut Vec<u8>){
    ram[cpu.get_bc() as usize] = cpu.get_a();
}

//0x03
pub fn inc_bc(cpu : &mut Cpu){
    cpu.set_bc(cpu.get_bc() + 0b1);
}

//0x04
pub fn inc_b(cpu :&mut Cpu){
    if cpu.get_b() == 0x0f { //Half carry flag
        cpu.set_flag(H);
    }else{
        cpu.clear_flag(H)
    }

    cpu.set_b(cpu.get_b().wrapping_add(1));

    if cpu.get_b() == 0 { // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x05
pub fn dec_b(cpu : &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_b() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
        //we have bits in the lower nibble
    }else{
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_b(cpu.get_b().wrapping_sub(1));

    cpu.set_flag(N); //setting N flag
}

//0xF3 (4tics)
pub fn di(cpu : &mut Cpu){
    cpu.set_mie(false);
}

//0xFB (4tics)
pub fn ei(cpu : &mut Cpu){
    cpu.set_mie(true);
}


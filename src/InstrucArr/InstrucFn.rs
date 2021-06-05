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
pub fn ld_bc_u16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_b(h);
    cpu.set_c(l);
}
//0x02
pub fn ld_bcp_a(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
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
/*****************8 bit direct load***********************/
//0x3E
pub fn  ld_a_u8(cpu : &mut Cpu, n : u8){
    cpu.set_a(n);
}
//0x06
pub fn  ld_b_u8(cpu : &mut Cpu, n : u8){
    cpu.set_b(n);
}
//0x0e
pub fn  ld_c_u8(cpu : &mut Cpu, n : u8){
    cpu.set_c(n);
}
//0x16
pub fn  ld_d_u8(cpu : &mut Cpu, n : u8){
    cpu.set_d(n);
}
//0x1e
pub fn  ld_e_u8(cpu : &mut Cpu, n : u8){
    cpu.set_e(n);
}
//0x26
pub fn  ld_h_u8(cpu : &mut Cpu, n : u8){
    cpu.set_h(n);
}
//0x2e
pub fn  ld_l_u8(cpu : &mut Cpu, n : u8){
    cpu.set_l(n);
}
/*****************Load A***********************/
//0x7f
pub fn  ld_a_a(cpu : &mut Cpu){
    cpu.set_a(cpu.get_a());
}
//0x78
pub fn  ld_a_b(cpu : &mut Cpu){
    cpu.set_a(cpu.get_b());
}
//0x79
pub fn  ld_a_c(cpu : &mut Cpu){
    cpu.set_a(cpu.get_c());
}
//0x7a
pub fn  ld_a_d(cpu : &mut Cpu){
    cpu.set_a(cpu.get_d());
}
//0x7b
pub fn  ld_a_e(cpu : &mut Cpu){
    cpu.set_a(cpu.get_e());
}
//0x7C
pub fn  ld_a_h(cpu : &mut Cpu){
    cpu.set_a(cpu.get_f());
}
//0x7D
pub fn  ld_a_l(cpu : &mut Cpu){
    cpu.set_a(cpu.get_l());
}
//0x0A
pub fn  ld_a_bcp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_a(ram[cpu.get_bc() as usize]);
}
//0x1A
pub fn  ld_a_dep(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_a(ram[cpu.get_de() as usize]);
}
//0x7E
pub fn  ld_a_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_a(ram[cpu.get_hl() as usize]);
}
//0xFA
pub fn  ld_a_u16p(cpu : &mut Cpu, h : u8,l : u8, ram : &mut [u8;0x10000]){
    cpu.set_a(ram[Cpu::get_u16(h,l) as usize]);
}

/*****************Load B***********************/
//0x40
pub fn ld_b_b(cpu : &mut Cpu){
    cpu.set_b(cpu.get_b());
}
//0x41
pub fn ld_b_c(cpu : &mut Cpu){
    cpu.set_b(cpu.get_c());
}
//0x42
pub fn ld_b_d(cpu : &mut Cpu){
    cpu.set_b(cpu.get_d());
}
//0x43
pub fn ld_b_e(cpu : &mut Cpu){
    cpu.set_b(cpu.get_e());
}
//0x44
pub fn ld_b_h(cpu : &mut Cpu){
    cpu.set_b(cpu.get_h());
}
//0x45
pub fn ld_b_l(cpu : &mut Cpu){
    cpu.set_b(cpu.get_l());
}
//0x46
pub fn  ld_b_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_b(ram[cpu.get_hl() as usize]);
}
//0x47
pub fn ld_b_a(cpu : &mut Cpu){
    cpu.set_b(cpu.get_a());
}
/*****************Load C***********************/
//0x48
pub fn ld_c_b(cpu : &mut Cpu){
    cpu.set_c(cpu.get_b());
}
//0x49
pub fn ld_c_c(cpu : &mut Cpu){
    cpu.set_c(cpu.get_c());
}
//0x4A
pub fn ld_c_d(cpu : &mut Cpu){
    cpu.set_c(cpu.get_d());
}
//0x4B
pub fn ld_c_e(cpu : &mut Cpu){
    cpu.set_c(cpu.get_e());
}
//0x4C
pub fn ld_c_h(cpu : &mut Cpu){
    cpu.set_c(cpu.get_h());
}
//0x4D
pub fn ld_c_l(cpu : &mut Cpu){
    cpu.set_c(cpu.get_l());
}
//0x4C
pub fn  ld_c_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_c(ram[cpu.get_hl() as usize]);
}
//0x4F
pub fn ld_c_a(cpu : &mut Cpu){
    cpu.set_c(cpu.get_a());
}
/*****************Load D***********************/
//0x50
pub fn ld_d_b(cpu : &mut Cpu){
    cpu.set_d(cpu.get_b());
}
//0x51
pub fn ld_d_c(cpu : &mut Cpu){
    cpu.set_d(cpu.get_c());
}
//0x52
pub fn ld_d_d(cpu : &mut Cpu){
    cpu.set_d(cpu.get_d());
}
//0x53
pub fn ld_d_e(cpu : &mut Cpu){
    cpu.set_d(cpu.get_e());
}
//0x54
pub fn ld_d_h(cpu : &mut Cpu){
    cpu.set_d(cpu.get_h());
}
//0x55
pub fn ld_d_l(cpu : &mut Cpu){
    cpu.set_d(cpu.get_l());
}
//0x56
pub fn  ld_d_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_d(ram[cpu.get_hl() as usize]);
}
//0x57
pub fn ld_d_a(cpu : &mut Cpu){
    cpu.set_d(cpu.get_a());
}
/*****************Load E***********************/
//0x58
pub fn ld_e_b(cpu : &mut Cpu){
    cpu.set_e(cpu.get_b());
}
//0x59
pub fn ld_e_c(cpu : &mut Cpu){
    cpu.set_e(cpu.get_c());
}
//0x5A
pub fn ld_e_d(cpu : &mut Cpu){
    cpu.set_e(cpu.get_d());
}
//0x5B
pub fn ld_e_e(cpu : &mut Cpu){
    cpu.set_e(cpu.get_e());
}
//0x5C
pub fn ld_e_h(cpu : &mut Cpu){
    cpu.set_e(cpu.get_h());
}
//0x5D
pub fn ld_e_l(cpu : &mut Cpu){
    cpu.set_e(cpu.get_l());
}
//0x5E
pub fn  ld_e_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_e(ram[cpu.get_hl() as usize]);
}
//0x5F
pub fn ld_e_a(cpu : &mut Cpu){
    cpu.set_e(cpu.get_a());
}
/*****************Load H***********************/
//0x60
pub fn ld_h_b(cpu : &mut Cpu){
    cpu.set_h(cpu.get_b());
}
//0x61
pub fn ld_h_c(cpu : &mut Cpu){
    cpu.set_h(cpu.get_c());
}
//0x62
pub fn ld_h_d(cpu : &mut Cpu){
    cpu.set_h(cpu.get_d());
}
//0x63
pub fn ld_h_e(cpu : &mut Cpu){
    cpu.set_h(cpu.get_e());
}
//0x64
pub fn ld_h_h(cpu : &mut Cpu){
    cpu.set_h(cpu.get_h());
}
//0x65
pub fn ld_h_l(cpu : &mut Cpu){
    cpu.set_h(cpu.get_l());
}
//0x66
pub fn  ld_h_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_h(ram[cpu.get_hl() as usize]);
}
//0x67
pub fn ld_h_a(cpu : &mut Cpu){
    cpu.set_h(cpu.get_a());
}
/*****************Load L***********************/
//0x68
pub fn ld_l_b(cpu : &mut Cpu){
    cpu.set_l(cpu.get_b());
}
//0x69
pub fn ld_l_c(cpu : &mut Cpu){
    cpu.set_l(cpu.get_c());
}
//0x6A
pub fn ld_l_d(cpu : &mut Cpu){
    cpu.set_l(cpu.get_d());
}
//0x6B
pub fn ld_l_e(cpu : &mut Cpu){
    cpu.set_l(cpu.get_e());
}
//0x6C
pub fn ld_l_h(cpu : &mut Cpu){
    cpu.set_l(cpu.get_h());
}
//0x6D
pub fn ld_l_l(cpu : &mut Cpu){
    cpu.set_l(cpu.get_l());
}
//0x6E
pub fn  ld_l_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_l(ram[cpu.get_hl() as usize]);
}
//0x6F
pub fn ld_l_a(cpu : &mut Cpu){
    cpu.set_l(cpu.get_a());
}
/*****************Load (HL)***********************/
//0x70
pub fn ld_hlp_b(cpu: &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_b(),cpu.get_hl());
}
//0x71
pub fn ld_hlp_c(cpu: &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_c(),cpu.get_hl());
}
//0x72
pub fn ld_hlp_d(cpu: &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_d(),cpu.get_hl());
}
//0x73
pub fn ld_hlp_e(cpu: &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_e(),cpu.get_hl());
}
//0x74
pub fn ld_hlp_h(cpu: &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_h(),cpu.get_hl());
}
//0x75
pub fn ld_hlp_l(cpu: &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_l(),cpu.get_hl());
}
//0x0x36
pub fn ld_hlp_u8(cpu: &mut Cpu, n : u8, ram : &mut [u8;0x10000]){
    cpu.write(ram,n,cpu.get_hl());
}
//0x77
pub fn ld_hlp_a(cpu: &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_a(),cpu.get_hl());
}


//0xF3 (4tics)
pub fn di(cpu : &mut Cpu){
    cpu.set_mie(false);
}

//0xFB (4tics)
pub fn ei(cpu : &mut Cpu){
    cpu.set_mie(true);
}

//0xD9
pub fn reti (cpu: &mut Cpu, ram: &mut [u8;0x10000]){
    cpu.set_mie(true);
    let pc : u16 = cpu.read_u16_from_stack(&ram);
    cpu.set_pc(pc);
}
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::Hardware;
use crate::Hardware::Cpu as Cpu;
use crate::Hardware::Flag::*;

//0x00
pub fn nop(cpu : &mut Cpu){}

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
    cpu.set_a(cpu.get_h());
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

/*****************Load n from A***********************/
//0x02
pub fn ld_bcp_a(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_a(),cpu.get_bc());
}
//0x12
pub fn ld_dep_a(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_a(),cpu.get_de());
}
//0xEA
pub fn ld_u16p_a(cpu :&mut Cpu,h : u8, l : u8, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_a(),Cpu::get_u16(h,l));
}
/*****************Load A and HRam+C***********************/
//0xF2
pub fn ld_a_hram_c(cpu :&mut Cpu,ram : &mut [u8;0x10000]){
    cpu.set_a(ram[(0xff00 + cpu.get_c() as u16) as usize]);
}
//0xE2
pub fn ld_hram_c_a(cpu :&mut Cpu,ram : &mut [u8;0x10000]){
    cpu.write(ram, cpu.get_a(), (0xff00 + cpu.get_c() as u16));
}
/*****************Load A and decrease/increase HL***********************/
//0x3A //TODO revoir le system de read?
pub fn ldd_a_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_a(ram[cpu.get_hl() as usize]);
    cpu.set_hl(cpu.get_hl().wrapping_sub(1));
}
//0x32
pub fn ldd_hlp_a(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_a(),cpu.get_hl());
    cpu.set_hl(cpu.get_hl().wrapping_sub(1));
}
//0x2A
pub fn ldi_a_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.set_a(ram[cpu.get_hl() as usize]);
    cpu.set_hl(cpu.get_hl().wrapping_add(1));
}
//0x22
pub fn ldi_hlp_a(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_a(),cpu.get_hl());
    cpu.set_hl(cpu.get_hl().wrapping_add(1));
}
/*****************Load A and HRam + n***********************/
//0xF0 //TODO revoir le system de read?
pub fn ldh_a_u8(cpu: &mut Cpu, n : u8, ram : &mut [u8;0x10000]){
    cpu.set_a(ram[(0xff00 + n as u16)as usize]);
}
//0xE0
pub fn ldh_u8_a(cpu: &mut Cpu, n : u8, ram : &mut [u8;0x10000]){
    cpu.write(ram,cpu.get_a(),0xff00 + n as u16);
}
/*****************16 bits direct loads***********************/
//0x01
pub fn ld_bc_u16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_b(h);
    cpu.set_c(l);
}
//0x11
pub fn ld_de_u16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_d(h);
    cpu.set_e(l);
}
//0x21
pub fn ld_hl_u16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_h(h);
    cpu.set_l(l);
}
//0x31
pub fn ld_sp_u16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_sp(Cpu::get_u16(h,l));
}
/*****************SP related loads***********************/
//0xF9
pub fn ld_sp_hl(cpu : &mut Cpu){
    cpu.set_sp(cpu.get_hl());
}
/*0xF8
Complex instruction, the Gameboy use Two's complement signed number
We use smartly rust cast function to avoid having to rotate the bits
 */
pub fn ld_hl_sp_i8(cpu : &mut Cpu, n: u8){

    match cpu.get_sp().checked_add(n as u16) { //checking for an overflow
        None => cpu.set_flag(C),
        Some(_) => cpu.clear_flag(C),
    }
    if (cpu.get_sp() & 0x0f) +  (n & 0x0f) as u16 > 0x0f {
        cpu.set_flag(H);
    }else{
        cpu.clear_flag(H);
    }

    cpu.clear_flag(Z);
    cpu.clear_flag(N);
    cpu.set_sp((cpu.get_sp() as i16).wrapping_add((n as i8) as i16) as u16);

}



//0xF3 (4tics)
pub fn di(cpu : &mut Cpu){
    cpu.set_mie(false);
}

//0xFB (4tics)
pub fn ei(cpu : &mut Cpu){
    //Since you there is a delay of instruction
    //before enabling interrupts it's handle in the Master::step method
}

//0xAF
pub fn xor_a(cpu :&mut Cpu){
    cpu.set_a(cpu.get_a() ^ cpu.get_a());
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.clear_flag(N);
    if cpu.get_a() == 0{
        cpu.set_flag(Z)
    }else{
        cpu.clear_flag(Z);
    }
}

pub fn xor_c(cpu :&mut Cpu){
    cpu.set_a(cpu.get_a() ^ cpu.get_c());
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.clear_flag(N);
    if cpu.get_a() == 0{
        cpu.set_flag(Z)
    }else{
        cpu.clear_flag(Z);
    }
}
/*****************JUMP***********************/
//0xC3
pub fn jp_u16(cpu : &mut Cpu, h : u8, l: u8){
    cpu.set_pc(Cpu::get_u16(h,l).wrapping_sub(3));
}
//0xC2
pub fn jp_nz_u16(cpu : &mut Cpu, h : u8, l: u8){
    if !cpu.get_flags().Z{
        jp_u16(cpu,h,l);
    }
}
//0xCA
pub fn jp_z_u16(cpu : &mut Cpu, h : u8, l: u8){
    if cpu.get_flags().Z{
        jp_u16(cpu,h,l);
    }
}
//0xD2
pub fn jp_nc_u16(cpu : &mut Cpu, h : u8, l: u8){
    if !cpu.get_flags().C{
        jp_u16(cpu,h,l);
    }
}
//0xDA
pub fn jp_c_u16(cpu : &mut Cpu, h : u8, l: u8){
    if cpu.get_flags().C{
        jp_u16(cpu,h,l);
    }
}
//0xE9
pub fn jp_hl(cpu : &mut Cpu){
    cpu.set_pc(cpu.get_hl().wrapping_sub(1));
}
//0x18
pub fn jpr(cpu : &mut Cpu, n: u8){
    cpu.set_pc((cpu.get_pc() as i16).wrapping_add((n as i8) as i16) as u16);
    //TODO: Timing ?
}

//Ox20
pub fn jpr_nz(cpu : &mut Cpu, n: u8){
    if !cpu.get_flags().Z {
        jpr(cpu, n);
    }
}
//0x28
pub fn jpr_z(cpu : &mut Cpu, n: u8){
    if cpu.get_flags().Z {
        jpr(cpu, n);
    }
}
//0x30
pub fn jpr_nc(cpu : &mut Cpu, n: u8){
    if !cpu.get_flags().C {
        jpr(cpu, n);
    }
}
//0x38
pub fn jpr_c(cpu : &mut Cpu, n: u8){
    if cpu.get_flags().C {
        jpr(cpu, n);
    }
}


/*DEC_____________________________________________________________________*/
//0x3D
pub fn dec_a(cpu : &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_a() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
        //we have bits in the lower nibble
    }else{
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_a(cpu.get_a().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
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

    cpu.set_flag(N);
    if cpu.get_b() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}

//0x0D
pub fn dec_c(cpu : &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_c() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
        //we have bits in the lower nibble
    }else{
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_c(cpu.get_c().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_c() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}

//0x15
pub fn dec_d(cpu : &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_d() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
        //we have bits in the lower nibble
    }else{
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_d(cpu.get_d().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_d() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}

//0x1D
pub fn dec_e(cpu : &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_e() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
        //we have bits in the lower nibble
    }else{
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_e(cpu.get_e().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_e() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}

//0x25
pub fn dec_h(cpu : &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_h() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
        //we have bits in the lower nibble
    }else{
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_h(cpu.get_h().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_h() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}

//0x2D
pub fn dec_l(cpu : &mut Cpu) {
    //Half carry flag here used as half borrow :

    if cpu.get_l() & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
        //we have bits in the lower nibble
    }else{
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    cpu.set_l(cpu.get_l().wrapping_sub(1));

    cpu.set_flag(N);
    if cpu.get_l() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}

//0x35
pub fn dec_hlp(cpu: &mut Cpu, ram: &mut [u8;0x10000]) {
    //Half carry flag here used as half borrow :

    if ram[cpu.get_hl() as usize] & 0x0f > 0 {
        cpu.clear_flag(H); //no borrow
        //we have bits in the lower nibble
    }else{
        cpu.set_flag(H);
        //we have no bits in the lower nibble
        //we have to "borrow" some bits of the lower nibble :
        //i.e 0xf0 - 0x01 = 0xef
    }

    ram[cpu.get_hl() as usize]=(ram[cpu.get_hl() as usize].wrapping_sub(1));

    cpu.set_flag(N);
    if ram[cpu.get_hl() as usize] == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}

//0xFE
pub fn cp_u8(cpu : &mut Cpu, n: u8){
    cpu.set_flag(N);
    if n > cpu.get_a() {
        cpu.set_flag(C);
    }else{
        cpu.clear_flag(C);
    }
    if cpu.get_a().wrapping_sub(n) == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
    if (n & 0x0f) > (cpu.get_a() & 0x0f){ //(like a - n) check for borrow
        cpu.set_flag(H);
    }else{
        cpu.clear_flag(H);
    }
}

/*INC__________________________________________________________________________________*/

//0x3C
pub fn inc_a(cpu :&mut Cpu){
    if cpu.get_a() == 0x0f { //Half carry flag
        cpu.set_flag(H);
    }else{
        cpu.clear_flag(H)
    }

    cpu.set_a(cpu.get_a().wrapping_add(1));

    if cpu.get_a() == 0 { // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
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
//0x0C
pub fn inc_c(cpu :&mut Cpu){
    if cpu.get_c() == 0x0f { //Half carry flag
        cpu.set_flag(H);
    }else{
        cpu.clear_flag(H)
    }

    cpu.set_c(cpu.get_c().wrapping_add(1));

    if cpu.get_c() == 0 { // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x14
pub fn inc_d(cpu :&mut Cpu){
    if cpu.get_d() == 0x0f { //Half carry flag
        cpu.set_flag(H);
    }else{
        cpu.clear_flag(H)
    }

    cpu.set_d(cpu.get_d().wrapping_add(1));

    if cpu.get_d() == 0 { // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x1C
pub fn inc_e(cpu :&mut Cpu){
    if cpu.get_e() == 0x0f { //Half carry flag
        cpu.set_flag(H);
    }else{
        cpu.clear_flag(H)
    }

    cpu.set_e(cpu.get_e().wrapping_add(1));

    if cpu.get_e() == 0 { // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x24
pub fn inc_h(cpu :&mut Cpu){
    if cpu.get_h() == 0x0f { //Half carry flag
        cpu.set_flag(H);
    }else{
        cpu.clear_flag(H)
    }

    cpu.set_h(cpu.get_h().wrapping_add(1));

    if cpu.get_h() == 0 { // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x2C
pub fn inc_l(cpu :&mut Cpu){
    if cpu.get_l() == 0x0f { //Half carry flag
        cpu.set_flag(H);
    }else{
        cpu.clear_flag(H)
    }

    cpu.set_l(cpu.get_l().wrapping_add(1));

    if cpu.get_l() == 0 { // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//0x34
pub fn inc_hlp(cpu: &mut Cpu, ram: &mut [u8;0x10000]){
    if ram[cpu.get_hl() as usize] == 0x0f { //Half carry flag
        cpu.set_flag(H);
    }else{
        cpu.clear_flag(H)
    }

    ram[cpu.get_hl() as usize] = ram[cpu.get_hl() as usize].wrapping_add(1);

    if ram[cpu.get_hl() as usize] == 0 { // Zero flag
        cpu.set_flag(Z);
    } else {
        cpu.clear_flag(Z)
    } //zero flag
    cpu.clear_flag(N); //clearing N flag
}
//TODO: timing
/*CALLS____________________________________________________________________________*/
//0xCD
pub fn call_u16(cpu :&mut Cpu,h : u8, l : u8, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_pc().wrapping_add(3),ram);
    cpu.set_pc(Cpu::get_u16(h,l).wrapping_sub(3));
}
//C4
pub fn call_nz_u16(cpu :&mut Cpu,h : u8, l : u8, ram : &mut [u8;0x10000]){
    if(!cpu.get_flags().Z){
        call_u16(cpu,h,l,ram);
    }
}
//CC
pub fn call_z_u16(cpu :&mut Cpu,h : u8, l : u8, ram : &mut [u8;0x10000]){
    if(cpu.get_flags().Z){
        call_u16(cpu,h,l,ram);
    }
}
//D4
pub fn call_nc_u16(cpu :&mut Cpu,h : u8, l : u8, ram : &mut [u8;0x10000]){
    if(!cpu.get_flags().C){
        call_u16(cpu,h,l,ram);
    }
}
//DC
pub fn call_c_u16(cpu :&mut Cpu,h : u8, l : u8, ram : &mut [u8;0x10000]){
    if(cpu.get_flags().C){
        call_u16(cpu,h,l,ram);
    }
}
/*****************DEC 16 bits***********************/
//0x0B
pub fn dec_bc(cpu : &mut Cpu){
    cpu.set_bc(cpu.get_bc().wrapping_sub(1));
}
//0x1B
pub fn dec_de(cpu : &mut Cpu){
    cpu.set_de(cpu.get_de().wrapping_sub(1));
}
//0x2B
pub fn dec_hl(cpu : &mut Cpu){
    cpu.set_hl(cpu.get_hl().wrapping_sub(1));
}
//0x3B
pub fn dec_sp(cpu : &mut Cpu){
    cpu.set_sp(cpu.get_sp().wrapping_sub(1));
}
/*****************INC 16 bits***********************/
//0x03
pub fn inc_bc(cpu : &mut Cpu){
    cpu.set_bc(cpu.get_bc().wrapping_add(1));
}
//0x13
pub fn inc_de(cpu : &mut Cpu){
    cpu.set_de(cpu.get_de().wrapping_add(1));
}
//0x23
pub fn inc_hl(cpu : &mut Cpu){
    cpu.set_hl(cpu.get_hl().wrapping_add(1));
}
//0x33
pub fn inc_sp(cpu : &mut Cpu){
    cpu.set_sp(cpu.get_sp().wrapping_add(1));
}
/*****************OR***********************/
//0xB7
pub fn or_a(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() | cpu.get_a());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xB0
pub fn or_b(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() | cpu.get_b());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xB1
pub fn or_c(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() | cpu.get_c());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xB2
pub fn or_d(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() | cpu.get_d());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xB3
pub fn or_e(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() | cpu.get_e());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xB4
pub fn or_h(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() | cpu.get_h());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xB5
pub fn or_l(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() | cpu.get_l());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xB6
pub fn or_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() | ram[cpu.get_hl() as usize]);
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xF6
pub fn or_u8(cpu : &mut Cpu, n : u8){
    cpu.clear_flag(N);
    cpu.clear_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() | n );
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
/*****************Returns***********************/
//0xC9 //TODO TIMING
pub fn ret(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    let n = cpu.read_u16_from_stack(ram).wrapping_sub(1);
    cpu.set_pc(n);
}
//0xC0
pub fn ret_nz(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    if !cpu.get_flags().Z {
        ret(cpu,ram);
    }
}
//0xC8
pub fn ret_z(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    if cpu.get_flags().Z {
        ret(cpu,ram);
    }
}
//0xD0
pub fn ret_nc(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    if !cpu.get_flags().C {
        ret(cpu,ram);
    }
}
//0xD8
pub fn ret_c(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    if cpu.get_flags().C {
        ret(cpu,ram);
    }
}
//0xD9
pub fn ret_i (cpu: &mut Cpu, ram: &mut [u8;0x10000]){
    cpu.set_mie(true);
    let pc : u16 = cpu.read_u16_from_stack(&ram).wrapping_sub(1);
    cpu.set_pc(pc);
}
/*****************SP related PUSH***********************/
//0xF5
pub fn push_af(cpu :&mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_af(),ram);
}
//0xC5
pub fn push_bc(cpu :&mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_bc(),ram);
}
//0xD5
pub fn push_de(cpu :&mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_de(),ram);
}
//0xE5
pub fn push_hl(cpu :&mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_hl(),ram);
}
/*****************SP related POP***********************/
//0xF1
pub fn pop_af(cpu :&mut Cpu, ram : &mut [u8;0x10000]){
    let n = cpu.read_u16_from_stack(ram);
    cpu.set_af(n);
}
//0xC1
pub fn pop_bc(cpu :&mut Cpu, ram : &mut [u8;0x10000]){
    let n = cpu.read_u16_from_stack(ram);
    cpu.set_bc(n);
}
//0xDE
pub fn pop_de(cpu :&mut Cpu, ram : &mut [u8;0x10000]){
    let n = cpu.read_u16_from_stack(ram);
    cpu.set_de(n);
}
//0xhl
pub fn pop_hl(cpu :&mut Cpu, ram : &mut [u8;0x10000]){
    let n = cpu.read_u16_from_stack(ram);
    cpu.set_hl(n);
}
/*****************AND***********************/
//0xA7
pub fn and_a(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.set_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() & cpu.get_a());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xA0
pub fn and_b(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.set_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() & cpu.get_b());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xA1
pub fn and_c(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.set_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() & cpu.get_c());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xA2
pub fn and_d(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.set_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() & cpu.get_d());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xA3
pub fn and_e(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.set_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() & cpu.get_e());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xA4
pub fn and_h(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.set_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() & cpu.get_h());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xA5
pub fn and_l(cpu : &mut Cpu){
    cpu.clear_flag(N);
    cpu.set_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() & cpu.get_l());
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xA6
pub fn and_hlp(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.clear_flag(N);
    cpu.set_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() & ram[cpu.get_hl() as usize]);
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//0xE6
pub fn and_u8(cpu : &mut Cpu, n : u8){
    cpu.clear_flag(N);
    cpu.set_flag(H);
    cpu.clear_flag(C);
    cpu.set_a(cpu.get_a() & n);
    if cpu.get_a() == 0{
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z);
    }
}
//MISC
//0x2F
pub fn cpl(cpu : &mut Cpu){
    cpu.set_flag(H);
    cpu.set_flag(N);
    cpu.set_a(!(cpu.get_a()));
}

//0xCB
pub fn prefix(cpu : &mut Cpu, n : u8){
    match n {
        0x37 =>{
            println!("/!\\ SWAP A operation occured");
            cpu.set_a((cpu.get_a() << 4) | (cpu.get_a() >> 4));
            cpu.set_flag(Z);
            cpu.clear_flag(H);
            cpu.clear_flag(C);
            cpu.clear_flag(N);
        },
        0x87 => {
            println!("/!\\ Res 0 A operation occured");
            cpu.set_a(cpu.get_a() & 0xfe);
        },
        _ => {
            println!("/!\\ {:#04x} Not done yet",n);
        }
    }
}

/*RESET_____________________________________________________________________*/
//C7
pub fn rst_0(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_pc() + 1,ram);
    cpu.set_pc(0xFFFF);
}
//CF
pub fn rst_8(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_pc() + 1,ram);
    cpu.set_pc(0x0007);
}

//D7
pub fn rst_10(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_pc() + 1,ram);
    cpu.set_pc(0x000f);
}

//DF
pub fn rst_18(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_pc() + 1,ram);
    cpu.set_pc(0x0017);
}
//E7
pub fn rst_20(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_pc() + 1,ram);
    cpu.set_pc(0x001f);
}

//EF
pub fn rst_28(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_pc() + 1,ram);
    cpu.set_pc(0x0027);
}

//F7
pub fn rst_30(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_pc() + 1,ram);
    cpu.set_pc(0x002f);
}

//FF
pub fn rst_38(cpu : &mut Cpu, ram : &mut [u8;0x10000]){
    cpu.write_u16_to_stack(cpu.get_pc() + 1,ram);
    cpu.set_pc(0x0037);
}
/*****************Add n to A***********************/
//0x87
pub fn add_a(cpu :&mut Cpu){
    add_u8(cpu,cpu.get_a());
}
//0x80
pub fn add_b(cpu :&mut Cpu){
    add_u8(cpu,cpu.get_b());
}
//0x81
pub fn add_c(cpu :&mut Cpu){
    add_u8(cpu,cpu.get_c());
}
//0x82
pub fn add_d(cpu :&mut Cpu){
    add_u8(cpu,cpu.get_d());
}
//0x83
pub fn add_e(cpu :&mut Cpu){
    add_u8(cpu,cpu.get_e());
}
//0x84
pub fn add_h(cpu :&mut Cpu){
    add_u8(cpu,cpu.get_h());
}
//0x85
pub fn add_l(cpu :&mut Cpu){
    add_u8(cpu,cpu.get_l());
}
//0x86
pub fn add_hlp(cpu :&mut Cpu, ram : &mut [u8;0x10000]){
    add_u8(cpu,ram[cpu.get_hl() as usize]);
}
//0xC6
pub fn add_u8(cpu :&mut Cpu, n : u8){
    match cpu.get_a().checked_add(n){
        None => {cpu.set_flag(C);}
        Some(_) => {cpu.clear_flag(C);}
    }
    if cpu.get_a() & 0x0f + n & 0x0f > 0x0f {
        cpu.set_flag(H);
    }else {
        cpu.clear_flag(H);
    }

    cpu.set_a(cpu.get_a().wrapping_add(n));
    if cpu.get_a() == 0 {
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z)
    }
    cpu.clear_flag(N);
}
/*****************ADD 16 bits***********************/
//Add_u16
fn add_hl_u16(cpu : &mut Cpu, n :u16){
    match cpu.get_hl().checked_add(n){
        None => {cpu.set_flag(C);}
        Some(_) => {cpu.clear_flag(C);}
    }
    if cpu.get_h() & 0x0f + ((n >>8 )as u8 & 0x0f) > 0x0f {
        cpu.set_flag(H);
    }else {
        cpu.clear_flag(H);
    }
    cpu.set_hl(cpu.get_hl().wrapping_add(n));
    if cpu.get_hl() == 0 {
        cpu.set_flag(Z);
    }else{
        cpu.clear_flag(Z)
    }
    cpu.clear_flag(N);
}
//0x09
pub fn add_hl_bc(cpu : &mut Cpu){
    add_hl_u16(cpu,cpu.get_bc());
}
//0x19
pub fn add_hl_de(cpu : &mut Cpu){
    add_hl_u16(cpu,cpu.get_de());
}
//0x29
pub fn add_hl_hl(cpu : &mut Cpu){
    add_hl_u16(cpu,cpu.get_hl());
}
//0x39
pub fn add_hl_sp(cpu : &mut Cpu){
    add_hl_u16(cpu,cpu.get_sp());
}

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

    //8 bit direct load
    out[0x3E] = Instruct::build_instruct(0x3E, String::from("LD A n"), String::from("Load n in A"), 1, 8, Op::u8(InstrucFn::ld_a_u8));
    out[0x06] = Instruct::build_instruct(0x06, String::from("LD B n"), String::from("Load n in B"), 1, 8, Op::u8(InstrucFn::ld_b_u8));
    out[0x0e] = Instruct::build_instruct(0x0e, String::from("LD C n"), String::from("Load n in C"), 1, 8, Op::u8(InstrucFn::ld_c_u8));
    out[0x16] = Instruct::build_instruct(0x16, String::from("LD D n"), String::from("Load n in D"), 1, 8, Op::u8(InstrucFn::ld_d_u8));
    out[0x1e] = Instruct::build_instruct(0x1e, String::from("LD E n"), String::from("Load n in E"), 1, 8, Op::u8(InstrucFn::ld_e_u8));
    out[0x26] = Instruct::build_instruct(0x26, String::from("LD H n"), String::from("Load n in H"), 1, 8, Op::u8(InstrucFn::ld_h_u8));
    out[0x2e] = Instruct::build_instruct(0x2e, String::from("LD L n "), String::from("Load n in L"), 1, 8, Op::u8(InstrucFn::ld_l_u8));
    //Load A
    out[0x7f] = Instruct::build_instruct(0x7f, String::from("LD A A"), String::from("Load A in A"), 0, 4, Op::no(InstrucFn::ld_a_a));
    out[0x78] = Instruct::build_instruct(0x78, String::from("LD A B"), String::from("Load B in A"), 0, 4, Op::no(InstrucFn::ld_a_b));
    out[0x79] = Instruct::build_instruct(0x79, String::from("LD A C"), String::from("Load C in A"), 0, 4, Op::no(InstrucFn::ld_a_c));
    out[0x7a] = Instruct::build_instruct(0x7a, String::from("LD A D"), String::from("Load D in A"), 0, 4, Op::no(InstrucFn::ld_a_d));
    out[0x7b] = Instruct::build_instruct(0x7b, String::from("LD A E"), String::from("Load E in A"), 0, 4, Op::no(InstrucFn::ld_a_e));
    out[0x7C] = Instruct::build_instruct(0x7C, String::from("LD A H"), String::from("Load H in A"), 0, 4, Op::no(InstrucFn::ld_a_h));
    out[0x7D] = Instruct::build_instruct(0x7D, String::from("LD A L"), String::from("Load L in A"), 0, 4, Op::no(InstrucFn::ld_a_l));
    out[0x0A] = Instruct::build_instruct(0x0A, String::from("LD A (BC)"), String::from("Load ram[BC] in A"), 0, 8, Op::ram(InstrucFn::ld_a_bcp));
    out[0x1A] = Instruct::build_instruct(0x1A, String::from("LD A (DE)"), String::from("Load ram[DE] in A"), 0, 8, Op::ram(InstrucFn::ld_a_dep));
    out[0x7E] = Instruct::build_instruct(0x7E, String::from("LD A (HL)"), String::from("Load ram[HL] in A"), 0, 8, Op::ram(InstrucFn::ld_a_hlp));
    out[0xFA] = Instruct::build_instruct(0xFA, String::from("LD A (nn)"), String::from("Load ram[nn] in A"), 2, 16, Op::ramu16(InstrucFn::ld_a_u16p));
    //Load B
    out[0x40] = Instruct::build_instruct(0x40, String::from("LD B B"), String::from("Load B in B"), 0, 4, Op::no(InstrucFn::ld_b_b));
    out[0x41] = Instruct::build_instruct(0x41, String::from("LD B C"), String::from("Load C in B"), 0, 4, Op::no(InstrucFn::ld_b_c));
    out[0x42] = Instruct::build_instruct(0x42, String::from("LD B D"), String::from("Load D in B"), 0, 4, Op::no(InstrucFn::ld_b_d));
    out[0x43] = Instruct::build_instruct(0x43, String::from("LD B E"), String::from("Load E in B"), 0, 4, Op::no(InstrucFn::ld_b_e));
    out[0x44] = Instruct::build_instruct(0x44, String::from("LD B H"), String::from("Load H in B"), 0, 4, Op::no(InstrucFn::ld_b_h));
    out[0x45] = Instruct::build_instruct(0x45, String::from("LD B L"), String::from("Load L in B"), 0, 4, Op::no(InstrucFn::ld_b_l));
    out[0x46] = Instruct::build_instruct(0x46, String::from("LD B (HL)"), String::from("Load ram[HL] in B"), 0, 8, Op::ram(InstrucFn::ld_b_hlp));
    out[0x47] = Instruct::build_instruct(0x47, String::from("LD B A"), String::from("Load A in B"), 0, 4, Op::no(InstrucFn::ld_b_a));
    //Load C
    out[0x48] = Instruct::build_instruct(0x48, String::from("LD C B"), String::from("Load B in C"), 0, 4, Op::no(InstrucFn::ld_c_b));
    out[0x49] = Instruct::build_instruct(0x49, String::from("LD C C"), String::from("Load C in C"), 0, 4, Op::no(InstrucFn::ld_c_c));
    out[0x4A] = Instruct::build_instruct(0x4A, String::from("LD C D"), String::from("Load D in C"), 0, 4, Op::no(InstrucFn::ld_c_d));
    out[0x4B] = Instruct::build_instruct(0x4B, String::from("LD C E"), String::from("Load E in C"), 0, 4, Op::no(InstrucFn::ld_c_e));
    out[0x4C] = Instruct::build_instruct(0x4C, String::from("LD C H"), String::from("Load H in C"), 0, 4, Op::no(InstrucFn::ld_c_h));
    out[0x4D] = Instruct::build_instruct(0x4D, String::from("LD C L"), String::from("Load L in C"), 0, 4, Op::no(InstrucFn::ld_c_l));
    out[0x4E] = Instruct::build_instruct(0x4E, String::from("LD C (HL)"), String::from("Load ram[HL] in C"), 0, 8, Op::ram(InstrucFn::ld_c_hlp));
    out[0x4f] = Instruct::build_instruct(0x4f, String::from("LD C A"), String::from("Load A in C"), 0, 4, Op::no(InstrucFn::ld_c_a));
    //Load D
    out[0x50] = Instruct::build_instruct(0x50, String::from("LD D B"), String::from("Load B in D"), 0, 4, Op::no(InstrucFn::ld_d_b));
    out[0x51] = Instruct::build_instruct(0x51, String::from("LD D C"), String::from("Load C in D"), 0, 4, Op::no(InstrucFn::ld_d_c));
    out[0x52] = Instruct::build_instruct(0x52, String::from("LD D D"), String::from("Load D in D"), 0, 4, Op::no(InstrucFn::ld_d_d));
    out[0x53] = Instruct::build_instruct(0x53, String::from("LD D E"), String::from("Load E in D"), 0, 4, Op::no(InstrucFn::ld_d_e));
    out[0x54] = Instruct::build_instruct(0x54, String::from("LD D H"), String::from("Load H in D"), 0, 4, Op::no(InstrucFn::ld_d_h));
    out[0x55] = Instruct::build_instruct(0x55, String::from("LD D L"), String::from("Load L in D"), 0, 4, Op::no(InstrucFn::ld_d_l));
    out[0x56] = Instruct::build_instruct(0x56, String::from("LD D (HL)"), String::from("Load ram[HL] in D"), 0, 8, Op::ram(InstrucFn::ld_d_hlp));
    out[0x57] = Instruct::build_instruct(0x57, String::from("LD D A"), String::from("Load A in D"), 0, 4, Op::no(InstrucFn::ld_d_a));
    //Load E
    out[0x58] = Instruct::build_instruct(0x58, String::from("LD E B"), String::from("Load B in E"), 0, 4, Op::no(InstrucFn::ld_e_b));
    out[0x59] = Instruct::build_instruct(0x59, String::from("LD E C"), String::from("Load C in E"), 0, 4, Op::no(InstrucFn::ld_e_c));
    out[0x5a] = Instruct::build_instruct(0x5a, String::from("LD E D"), String::from("Load D in E"), 0, 4, Op::no(InstrucFn::ld_e_d));
    out[0x5b] = Instruct::build_instruct(0x5b, String::from("LD E E"), String::from("Load E in E"), 0, 4, Op::no(InstrucFn::ld_e_e));
    out[0x5c] = Instruct::build_instruct(0x5c, String::from("LD E H"), String::from("Load H in E"), 0, 4, Op::no(InstrucFn::ld_e_h));
    out[0x5d] = Instruct::build_instruct(0x5d, String::from("LD E L"), String::from("Load L in E"), 0, 4, Op::no(InstrucFn::ld_e_l));
    out[0x5e] = Instruct::build_instruct(0x5e, String::from("LD E (HL)"), String::from("Load ram[HL] in E"), 0, 8, Op::ram(InstrucFn::ld_e_hlp));
    out[0x5F] = Instruct::build_instruct(0x5F, String::from("LD E A"), String::from("Load A in E"), 0, 4, Op::no(InstrucFn::ld_e_a));
    //Load H
    out[0x60] = Instruct::build_instruct(0x60, String::from("LD H B"), String::from("Load B in H"), 0, 4, Op::no(InstrucFn::ld_h_b));
    out[0x61] = Instruct::build_instruct(0x61, String::from("LD H C"), String::from("Load C in H"), 0, 4, Op::no(InstrucFn::ld_h_c));
    out[0x62] = Instruct::build_instruct(0x62, String::from("LD H D"), String::from("Load D in H"), 0, 4, Op::no(InstrucFn::ld_h_d));
    out[0x63] = Instruct::build_instruct(0x63, String::from("LD H E"), String::from("Load E in H"), 0, 4, Op::no(InstrucFn::ld_h_e));
    out[0x64] = Instruct::build_instruct(0x64, String::from("LD H H"), String::from("Load H in H"),0, 4, Op::no(InstrucFn::ld_h_h));
    out[0x65] = Instruct::build_instruct(0x65, String::from("LD H L"), String::from("Load L in H"),0, 4, Op::no(InstrucFn::ld_h_l));
    out[0x66] = Instruct::build_instruct(0x66, String::from("LD H (HL)"), String::from("Load ram[HL] in H"), 0, 8, Op::ram(InstrucFn::ld_h_hlp));
    out[0x67] = Instruct::build_instruct(0x67, String::from("LD H A "), String::from("Load A in H"), 0, 4, Op::no(InstrucFn::ld_h_a));
    //Load L
    out[0x68] = Instruct::build_instruct(0x68, String::from("LD L B"), String::from("Load B in L"),0, 4, Op::no(InstrucFn::ld_l_b));
    out[0x69] = Instruct::build_instruct(0x69, String::from("LD L C"), String::from("Load C in L"),0, 4, Op::no(InstrucFn::ld_l_c));
    out[0x6a] = Instruct::build_instruct(0x6a, String::from("LD L "), String::from("Load D in L"),0, 4, Op::no(InstrucFn::ld_l_d));
    out[0x6b] = Instruct::build_instruct(0x6b, String::from("LD L E"), String::from("Load E in L"),0, 4, Op::no(InstrucFn::ld_l_e));
    out[0x6c] = Instruct::build_instruct(0x6c, String::from("LD L H"), String::from("Load H in L"),0, 4, Op::no(InstrucFn::ld_l_h));
    out[0x6d] = Instruct::build_instruct(0x6d, String::from("LD L L"), String::from("Load L in L"),0, 4, Op::no(InstrucFn::ld_l_l));
    out[0x6e] = Instruct::build_instruct(0x6e, String::from("LD L (HL)"), String::from("Load ram[HL] in L"), 0, 8, Op::ram(InstrucFn::ld_l_hlp));
    out[0x6f] = Instruct::build_instruct(0x6f, String::from("LD L A"), String::from("Load A in L"), 0, 4, Op::no(InstrucFn::ld_l_a));
    //Load (HL)
    out[0x70] = Instruct::build_instruct(0x70, String::from("LD (HL) B"), String::from("Load B in ram[HL]"), 0, 8, Op::ram(InstrucFn::ld_hlp_b));
    out[0x71] = Instruct::build_instruct(0x71, String::from("LD (HL) C"), String::from("Load C in ram[HL]"), 0, 8, Op::ram(InstrucFn::ld_hlp_c));
    out[0x72] = Instruct::build_instruct(0x72, String::from("LD (HL) D"), String::from("Load D in ram[HL]"), 0, 8, Op::ram(InstrucFn::ld_hlp_d));
    out[0x73] = Instruct::build_instruct(0x73, String::from("LD (HL) E"), String::from("Load E in ram[HL]"), 0, 8, Op::ram(InstrucFn::ld_hlp_e));
    out[0x74] = Instruct::build_instruct(0x74, String::from("LD (HL) H"), String::from("Load H in ram[HL]"), 0, 8, Op::ram(InstrucFn::ld_hlp_h));
    out[0x75] = Instruct::build_instruct(0x75, String::from("LD (HL) L"), String::from("Load L in ram[HL]"), 0, 8, Op::ram(InstrucFn::ld_hlp_l));
    out[0x36] = Instruct::build_instruct(0x36, String::from("LD (HL) n"), String::from("Load n in ram[HL]"), 1, 12, Op::ramu8(InstrucFn::ld_hlp_u8));
    out[0x77] = Instruct::build_instruct(0x77, String::from("LD (HL) A"), String::from("Load A in ram[HL]"), 0, 8, Op::ram(InstrucFn::ld_hlp_a));
    //Load n from A
    out[0x02] = Instruct::build_instruct(0x02, String::from("LD (BC) A"), String::from("Load A in ram[BC]"), 0, 8, Op::ram(InstrucFn::ld_bcp_a));
    out[0x12] = Instruct::build_instruct(0x12, String::from("LD (DE) A"), String::from("Load A in ram[DE]"), 0, 8, Op::ram(InstrucFn::ld_dep_a));
    out[0xEA] = Instruct::build_instruct(0xEA, String::from("LD (nn) A"), String::from("Load A in ram[nn]"), 2, 16, Op::ramu16(InstrucFn::ld_u16p_a));
    //Load A and HRam+C
    out[0xF2] = Instruct::build_instruct(0xF2, String::from("LD A (C)"), String::from("Load ram[0xFF00 + C] in A"), 0, 8, Op::ram(InstrucFn::ld_a_hram_c));
    out[0xE2] = Instruct::build_instruct(0xE2, String::from("LD (C) A"), String::from("Load A in ram[0xFF00 + C]"), 0, 8, Op::ram(InstrucFn::ld_hram_c_a));
    //Load A and decrease/increase HL
    out[0x3A] = Instruct::build_instruct(0x3A, String::from("LDD A (HL)"), String::from("Load ram[HL] in A, HL--"), 0, 8, Op::ram(InstrucFn::ldd_a_hlp));
    out[0x32] = Instruct::build_instruct(0x32, String::from("LDD (HL) A"), String::from("Load A in ram[HL], HL--"), 0, 8, Op::ram(InstrucFn::ldd_hlp_a));
    out[0x2A] = Instruct::build_instruct(0x2A, String::from("LDI A (HL)"), String::from("Load ram[HL] in A, HL++"), 0, 8, Op::ram(InstrucFn::ldi_a_hlp));
    out[0x22] = Instruct::build_instruct(0x22, String::from("LDI (HL) A"), String::from("Load A in ram[HL], HL++"), 0, 8, Op::ram(InstrucFn::ldi_hlp_a));
    //Load A and HRam + n
    out[0xF0] = Instruct::build_instruct(0xF0, String::from("LDH A (n)"), String::from("Load ram[0xff00+n] in A"), 1, 12, Op::ramu8(InstrucFn::ldh_a_u8));
    out[0xE0] = Instruct::build_instruct(0xE0, String::from("LDH (n) A"), String::from("Load A in ram[0xff00+n]"), 1, 12, Op::ramu8(InstrucFn::ldh_u8_a));
    //16 bits direct loads
    out[0x01] = Instruct::build_instruct(0x01, String::from("LD BC nn /!\\"), String::from("Load nn in BC"), 2, 12, Op::u16(InstrucFn::ld_bc_u16));
    out[0x11] = Instruct::build_instruct(0x11, String::from("LD DE nn /!\\"), String::from("Load nn in DE"), 2, 12, Op::no(InstrucFn::nop));
    out[0x21] = Instruct::build_instruct(0x21, String::from("LD HL nn /!\\"), String::from("Load nn in HL"), 2, 12, Op::no(InstrucFn::nop));
    out[0x31] = Instruct::build_instruct(0x31, String::from("LD SP nn /!\\"), String::from("Load nn in SP"), 2, 12, Op::no(InstrucFn::nop));
    //SP related loads
    out[0xF9] = Instruct::build_instruct(0xF9, String::from("LD SP HL /!\\"), String::from("Load HL in SP"), 0, 8, Op::no(InstrucFn::nop));
    out[0xF8] = Instruct::build_instruct(0xF8, String::from("LDHL SP n /!\\"), String::from("Load SP+n in HL"), 1, 12, Op::no(InstrucFn::nop));
    out[0x08] = Instruct::build_instruct(0x08, String::from("LD (nn) SP /!\\"), String::from("Load SP in ram[nn]"), 2, 12, Op::no(InstrucFn::nop));
    //SP related PUSH
    out[0xF5] = Instruct::build_instruct(0xF5, String::from("PUSH AF /!\\"), String::from("Push AF in stack, SP--*2"), 0, 16, Op::no(InstrucFn::nop));
    out[0xC5] = Instruct::build_instruct(0xC5, String::from("PUSH BC /!\\"), String::from("Push BC in stack, SP--*2"), 0, 16, Op::no(InstrucFn::nop));
    out[0xD5] = Instruct::build_instruct(0xD5, String::from("PUSH DE /!\\"), String::from("Push DE in stack, SP--*2"), 0, 16, Op::no(InstrucFn::nop));
    out[0xE5] = Instruct::build_instruct(0xE5, String::from("PUSH HL /!\\"), String::from("Push HL in stack, SP--*2"), 0, 16, Op::no(InstrucFn::nop));
    //SP related POP
    out[0xF1] = Instruct::build_instruct(0xF1, String::from("POP AF /!\\"), String::from("Pop from stack in AF , SP++*2"), 0, 12, Op::no(InstrucFn::nop));
    out[0xC1] = Instruct::build_instruct(0xC1, String::from("POP BC /!\\"), String::from("Pop from stack in BC , SP++*2"), 0, 12, Op::no(InstrucFn::nop));
    out[0xD1] = Instruct::build_instruct(0xD1, String::from("POP DE /!\\"), String::from("Pop from stack in DE , SP++*2"), 0, 12, Op::no(InstrucFn::nop));
    out[0xE1] = Instruct::build_instruct(0xE1, String::from("POP HL /!\\"), String::from("Pop from stack in HL , SP++*2"), 0, 12, Op::no(InstrucFn::nop));
    //Add n to A
    out[0x87] = Instruct::build_instruct(0x87, String::from("ADD A A /!\\"), String::from("Add A to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x80] = Instruct::build_instruct(0x80, String::from("ADD A B /!\\"), String::from("Add B to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x81] = Instruct::build_instruct(0x81, String::from("ADD A C /!\\"), String::from("Add C to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x82] = Instruct::build_instruct(0x82, String::from("ADD A D /!\\"), String::from("Add D to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x83] = Instruct::build_instruct(0x83, String::from("ADD A E /!\\"), String::from("Add E to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x84] = Instruct::build_instruct(0x84, String::from("ADD A H /!\\"), String::from("Add H to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x85] = Instruct::build_instruct(0x85, String::from("ADD A L /!\\"), String::from("Add L to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x86] = Instruct::build_instruct(0x86, String::from("ADD A (HL) /!\\"), String::from("Add ram[HL] to A"), 0, 8, Op::no(InstrucFn::nop));
    out[0xC6] = Instruct::build_instruct(0xC6, String::from("ADD A n /!\\"), String::from("Add n to A"), 1, 8, Op::no(InstrucFn::nop));
    //Add n + carry flag to A
    out[0x8F] = Instruct::build_instruct(0x8F, String::from("ADC A A /!\\"), String::from("Add A + Cflag to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x88] = Instruct::build_instruct(0x88, String::from("ADC A B /!\\"), String::from("Add B + Cflag to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x89] = Instruct::build_instruct(0x89, String::from("ADC A C /!\\"), String::from("Add C + Cflag to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x8A] = Instruct::build_instruct(0x8A, String::from("ADC A D /!\\"), String::from("Add D + Cflag to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x8B] = Instruct::build_instruct(0x8B, String::from("ADC A E /!\\"), String::from("Add E + Cflag to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x8C] = Instruct::build_instruct(0x8C, String::from("ADC A H /!\\"), String::from("Add H + Cflag to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x8D] = Instruct::build_instruct(0x8D, String::from("ADC A L /!\\"), String::from("Add L + Cflag to A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x8E] = Instruct::build_instruct(0x8E, String::from("ADC A (HL) /!\\"), String::from("Add ram[HL] + Cflag to A"), 0, 8, Op::no(InstrucFn::nop));
    out[0xCE] = Instruct::build_instruct(0xCE, String::from("ADC A n /!\\"), String::from("Add n + Cflag to A"), 1, 8, Op::no(InstrucFn::nop));
    //Sub from A
    out[0x97] = Instruct::build_instruct(0x97, String::from("SUB A /!\\"), String::from("Sub A from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x90] = Instruct::build_instruct(0x90, String::from("SUB B /!\\"), String::from("Sub B from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x91] = Instruct::build_instruct(0x91, String::from("SUB C /!\\"), String::from("Sub C from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x92] = Instruct::build_instruct(0x92, String::from("SUB D /!\\"), String::from("Sub D from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x93] = Instruct::build_instruct(0x93, String::from("SUB E /!\\"), String::from("Sub E from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x94] = Instruct::build_instruct(0x94, String::from("SUB H /!\\"), String::from("Sub H from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x95] = Instruct::build_instruct(0x95, String::from("SUB L /!\\"), String::from("Sub L from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x96] = Instruct::build_instruct(0x96, String::from("SUB (HL) /!\\"), String::from("Sub ram[HL] from A"), 0, 8, Op::no(InstrucFn::nop));
    out[0xD6] = Instruct::build_instruct(0xD6, String::from("SUB n /!\\"), String::from("Sub n from A"), 1, 8, Op::no(InstrucFn::nop));
    //Sub n + carry flag from A
    out[0x9F] = Instruct::build_instruct(0x9F, String::from("SBC A /!\\"), String::from("Sub A + Cflag from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x98] = Instruct::build_instruct(0x98, String::from("SBC B /!\\"), String::from("Sub B + Cflag from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x99] = Instruct::build_instruct(0x99, String::from("SBC C /!\\"), String::from("Sub C + Cflag from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x9A] = Instruct::build_instruct(0x9A, String::from("SBC D /!\\"), String::from("Sub D + Cflag from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x9B] = Instruct::build_instruct(0x9B, String::from("SBC E /!\\"), String::from("Sub E + Cflag from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x9C] = Instruct::build_instruct(0x9C, String::from("SBC H /!\\"), String::from("Sub H + Cflag from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x9D] = Instruct::build_instruct(0x9D, String::from("SBC L /!\\"), String::from("Sub L + Cflag from A"), 0, 4, Op::no(InstrucFn::nop));
    out[0x9E] = Instruct::build_instruct(0x9E, String::from("SBC (HL) /!\\"), String::from("Sub ram[HL] + Cflag from A"), 0, 8, Op::no(InstrucFn::nop));


    //What's left
    out[0x03] = Instruct::build_instruct(3, String::from("INC BC"), String::from("Increase BC "), 0, 8, Op::no(InstrucFn::inc_bc));
    out[0x04] = Instruct::build_instruct(4, String::from("INC B"), String::from("Increase B, manipulate Z,H flag, set N to 0"), 0, 4, Op::no(InstrucFn::inc_b));
    out[0x05] = Instruct::build_instruct(5, String::from("DEC B"), String::from("Decrease B, manipulate Z,H flag, set N to 1"), 0, 4, Op::no(InstrucFn::dec_b));
    out[0x06] = Instruct::build_instruct(6, String::from("LD B d8"), String::from("Load the d8 given in the operand in B"), 1, 8, Op::u8(InstrucFn::ld_b_u8));
    out[0xD9] = Instruct::build_instruct(0xD9, String::from("RETI"), String::from("Load the sp back to pc (return from interrupt)"), 0, 16, Op::ram(InstrucFn::reti));
    return out;


}

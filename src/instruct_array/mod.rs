use crate::hardware;
use hardware::Instruct;
use hardware::Op;

mod instruct_fn;

pub fn create_operations() -> Vec<Instruct> {
    let mut out = Vec::new();
    for i in 0..256 {
        out.push(Instruct {
            opcode: i,
            name: String::from("NOP"),
            desc: String::from("Aussi inutile que les cours de GE00"),
            argc: 0,
            ticks: 4,
            exec: Op::No(instruct_fn::nop),
        })
    }

    //8 bit direct load
    out[0x3E] = Instruct::build_instruct(0x3E, String::from("LD A n"), String::from("Load n in A"), 1, 8, Op::U8(instruct_fn::ld_a_u8));
    out[0x06] = Instruct::build_instruct(0x06, String::from("LD B n"), String::from("Load n in B"), 1, 8, Op::U8(instruct_fn::ld_b_u8));
    out[0x0e] = Instruct::build_instruct(0x0e, String::from("LD C n"), String::from("Load n in C"), 1, 8, Op::U8(instruct_fn::ld_c_u8));
    out[0x16] = Instruct::build_instruct(0x16, String::from("LD D n"), String::from("Load n in D"), 1, 8, Op::U8(instruct_fn::ld_d_u8));
    out[0x1e] = Instruct::build_instruct(0x1e, String::from("LD E n"), String::from("Load n in E"), 1, 8, Op::U8(instruct_fn::ld_e_u8));
    out[0x26] = Instruct::build_instruct(0x26, String::from("LD H n"), String::from("Load n in H"), 1, 8, Op::U8(instruct_fn::ld_h_u8));
    out[0x2e] = Instruct::build_instruct(0x2e, String::from("LD L n "), String::from("Load n in L"), 1, 8, Op::U8(instruct_fn::ld_l_u8));
    //Load A
    out[0x7f] = Instruct::build_instruct(0x7f, String::from("LD A A"), String::from("Load A in A"), 0, 4, Op::No(instruct_fn::ld_a_a));
    out[0x78] = Instruct::build_instruct(0x78, String::from("LD A B"), String::from("Load B in A"), 0, 4, Op::No(instruct_fn::ld_a_b));
    out[0x79] = Instruct::build_instruct(0x79, String::from("LD A C"), String::from("Load C in A"), 0, 4, Op::No(instruct_fn::ld_a_c));
    out[0x7a] = Instruct::build_instruct(0x7a, String::from("LD A D"), String::from("Load D in A"), 0, 4, Op::No(instruct_fn::ld_a_d));
    out[0x7b] = Instruct::build_instruct(0x7b, String::from("LD A E"), String::from("Load E in A"), 0, 4, Op::No(instruct_fn::ld_a_e));
    out[0x7C] = Instruct::build_instruct(0x7C, String::from("LD A H"), String::from("Load H in A"), 0, 4, Op::No(instruct_fn::ld_a_h));
    out[0x7D] = Instruct::build_instruct(0x7D, String::from("LD A L"), String::from("Load L in A"), 0, 4, Op::No(instruct_fn::ld_a_l));
    out[0x0A] = Instruct::build_instruct(0x0A, String::from("LD A (BC)"), String::from("Load ram[BC] in A"), 0, 8, Op::Ram(instruct_fn::ld_a_bcp));
    out[0x1A] = Instruct::build_instruct(0x1A, String::from("LD A (DE)"), String::from("Load ram[DE] in A"), 0, 8, Op::Ram(instruct_fn::ld_a_dep));
    out[0x7E] = Instruct::build_instruct(0x7E, String::from("LD A (HL)"), String::from("Load ram[HL] in A"), 0, 8, Op::Ram(instruct_fn::ld_a_hlp));
    out[0xFA] = Instruct::build_instruct(0xFA, String::from("LD A (nn)"), String::from("Load ram[nn] in A"), 2, 16, Op::RamU16(instruct_fn::ld_a_u16p));
    //Load B
    out[0x40] = Instruct::build_instruct(0x40, String::from("LD B B"), String::from("Load B in B"), 0, 4, Op::No(instruct_fn::ld_b_b));
    out[0x41] = Instruct::build_instruct(0x41, String::from("LD B C"), String::from("Load C in B"), 0, 4, Op::No(instruct_fn::ld_b_c));
    out[0x42] = Instruct::build_instruct(0x42, String::from("LD B D"), String::from("Load D in B"), 0, 4, Op::No(instruct_fn::ld_b_d));
    out[0x43] = Instruct::build_instruct(0x43, String::from("LD B E"), String::from("Load E in B"), 0, 4, Op::No(instruct_fn::ld_b_e));
    out[0x44] = Instruct::build_instruct(0x44, String::from("LD B H"), String::from("Load H in B"), 0, 4, Op::No(instruct_fn::ld_b_h));
    out[0x45] = Instruct::build_instruct(0x45, String::from("LD B L"), String::from("Load L in B"), 0, 4, Op::No(instruct_fn::ld_b_l));
    out[0x46] = Instruct::build_instruct(0x46, String::from("LD B (HL)"), String::from("Load ram[HL] in B"), 0, 8, Op::Ram(instruct_fn::ld_b_hlp));
    out[0x47] = Instruct::build_instruct(0x47, String::from("LD B A"), String::from("Load A in B"), 0, 4, Op::No(instruct_fn::ld_b_a));
    //Load C
    out[0x48] = Instruct::build_instruct(0x48, String::from("LD C B"), String::from("Load B in C"), 0, 4, Op::No(instruct_fn::ld_c_b));
    out[0x49] = Instruct::build_instruct(0x49, String::from("LD C C"), String::from("Load C in C"), 0, 4, Op::No(instruct_fn::ld_c_c));
    out[0x4A] = Instruct::build_instruct(0x4A, String::from("LD C D"), String::from("Load D in C"), 0, 4, Op::No(instruct_fn::ld_c_d));
    out[0x4B] = Instruct::build_instruct(0x4B, String::from("LD C E"), String::from("Load E in C"), 0, 4, Op::No(instruct_fn::ld_c_e));
    out[0x4C] = Instruct::build_instruct(0x4C, String::from("LD C H"), String::from("Load H in C"), 0, 4, Op::No(instruct_fn::ld_c_h));
    out[0x4D] = Instruct::build_instruct(0x4D, String::from("LD C L"), String::from("Load L in C"), 0, 4, Op::No(instruct_fn::ld_c_l));
    out[0x4E] = Instruct::build_instruct(0x4E, String::from("LD C (HL)"), String::from("Load ram[HL] in C"), 0, 8, Op::Ram(instruct_fn::ld_c_hlp));
    out[0x4f] = Instruct::build_instruct(0x4f, String::from("LD C A"), String::from("Load A in C"), 0, 4, Op::No(instruct_fn::ld_c_a));
    //Load D
    out[0x50] = Instruct::build_instruct(0x50, String::from("LD D B"), String::from("Load B in D"), 0, 4, Op::No(instruct_fn::ld_d_b));
    out[0x51] = Instruct::build_instruct(0x51, String::from("LD D C"), String::from("Load C in D"), 0, 4, Op::No(instruct_fn::ld_d_c));
    out[0x52] = Instruct::build_instruct(0x52, String::from("LD D D"), String::from("Load D in D"), 0, 4, Op::No(instruct_fn::ld_d_d));
    out[0x53] = Instruct::build_instruct(0x53, String::from("LD D E"), String::from("Load E in D"), 0, 4, Op::No(instruct_fn::ld_d_e));
    out[0x54] = Instruct::build_instruct(0x54, String::from("LD D H"), String::from("Load H in D"), 0, 4, Op::No(instruct_fn::ld_d_h));
    out[0x55] = Instruct::build_instruct(0x55, String::from("LD D L"), String::from("Load L in D"), 0, 4, Op::No(instruct_fn::ld_d_l));
    out[0x56] = Instruct::build_instruct(0x56, String::from("LD D (HL)"), String::from("Load ram[HL] in D"), 0, 8, Op::Ram(instruct_fn::ld_d_hlp));
    out[0x57] = Instruct::build_instruct(0x57, String::from("LD D A"), String::from("Load A in D"), 0, 4, Op::No(instruct_fn::ld_d_a));
    //Load E
    out[0x58] = Instruct::build_instruct(0x58, String::from("LD E B"), String::from("Load B in E"), 0, 4, Op::No(instruct_fn::ld_e_b));
    out[0x59] = Instruct::build_instruct(0x59, String::from("LD E C"), String::from("Load C in E"), 0, 4, Op::No(instruct_fn::ld_e_c));
    out[0x5a] = Instruct::build_instruct(0x5a, String::from("LD E D"), String::from("Load D in E"), 0, 4, Op::No(instruct_fn::ld_e_d));
    out[0x5b] = Instruct::build_instruct(0x5b, String::from("LD E E"), String::from("Load E in E"), 0, 4, Op::No(instruct_fn::ld_e_e));
    out[0x5c] = Instruct::build_instruct(0x5c, String::from("LD E H"), String::from("Load H in E"), 0, 4, Op::No(instruct_fn::ld_e_h));
    out[0x5d] = Instruct::build_instruct(0x5d, String::from("LD E L"), String::from("Load L in E"), 0, 4, Op::No(instruct_fn::ld_e_l));
    out[0x5e] = Instruct::build_instruct(0x5e, String::from("LD E (HL)"), String::from("Load ram[HL] in E"), 0, 8, Op::Ram(instruct_fn::ld_e_hlp));
    out[0x5F] = Instruct::build_instruct(0x5F, String::from("LD E A"), String::from("Load A in E"), 0, 4, Op::No(instruct_fn::ld_e_a));
    //Load H
    out[0x60] = Instruct::build_instruct(0x60, String::from("LD H B"), String::from("Load B in H"), 0, 4, Op::No(instruct_fn::ld_h_b));
    out[0x61] = Instruct::build_instruct(0x61, String::from("LD H C"), String::from("Load C in H"), 0, 4, Op::No(instruct_fn::ld_h_c));
    out[0x62] = Instruct::build_instruct(0x62, String::from("LD H D"), String::from("Load D in H"), 0, 4, Op::No(instruct_fn::ld_h_d));
    out[0x63] = Instruct::build_instruct(0x63, String::from("LD H E"), String::from("Load E in H"), 0, 4, Op::No(instruct_fn::ld_h_e));
    out[0x64] = Instruct::build_instruct(0x64, String::from("LD H H"), String::from("Load H in H"),0, 4, Op::No(instruct_fn::ld_h_h));
    out[0x65] = Instruct::build_instruct(0x65, String::from("LD H L"), String::from("Load L in H"),0, 4, Op::No(instruct_fn::ld_h_l));
    out[0x66] = Instruct::build_instruct(0x66, String::from("LD H (HL)"), String::from("Load ram[HL] in H"), 0, 8, Op::Ram(instruct_fn::ld_h_hlp));
    out[0x67] = Instruct::build_instruct(0x67, String::from("LD H A "), String::from("Load A in H"), 0, 4, Op::No(instruct_fn::ld_h_a));
    //Load L
    out[0x68] = Instruct::build_instruct(0x68, String::from("LD L B"), String::from("Load B in L"),0, 4, Op::No(instruct_fn::ld_l_b));
    out[0x69] = Instruct::build_instruct(0x69, String::from("LD L C"), String::from("Load C in L"),0, 4, Op::No(instruct_fn::ld_l_c));
    out[0x6a] = Instruct::build_instruct(0x6a, String::from("LD L "), String::from("Load D in L"),0, 4, Op::No(instruct_fn::ld_l_d));
    out[0x6b] = Instruct::build_instruct(0x6b, String::from("LD L E"), String::from("Load E in L"),0, 4, Op::No(instruct_fn::ld_l_e));
    out[0x6c] = Instruct::build_instruct(0x6c, String::from("LD L H"), String::from("Load H in L"),0, 4, Op::No(instruct_fn::ld_l_h));
    out[0x6d] = Instruct::build_instruct(0x6d, String::from("LD L L"), String::from("Load L in L"),0, 4, Op::No(instruct_fn::ld_l_l));
    out[0x6e] = Instruct::build_instruct(0x6e, String::from("LD L (HL)"), String::from("Load ram[HL] in L"), 0, 8, Op::Ram(instruct_fn::ld_l_hlp));
    out[0x6f] = Instruct::build_instruct(0x6f, String::from("LD L A"), String::from("Load A in L"), 0, 4, Op::No(instruct_fn::ld_l_a));
    //Load (HL)
    out[0x70] = Instruct::build_instruct(0x70, String::from("LD (HL) B"), String::from("Load B in ram[HL]"), 0, 8, Op::Ram(instruct_fn::ld_hlp_b));
    out[0x71] = Instruct::build_instruct(0x71, String::from("LD (HL) C"), String::from("Load C in ram[HL]"), 0, 8, Op::Ram(instruct_fn::ld_hlp_c));
    out[0x72] = Instruct::build_instruct(0x72, String::from("LD (HL) D"), String::from("Load D in ram[HL]"), 0, 8, Op::Ram(instruct_fn::ld_hlp_d));
    out[0x73] = Instruct::build_instruct(0x73, String::from("LD (HL) E"), String::from("Load E in ram[HL]"), 0, 8, Op::Ram(instruct_fn::ld_hlp_e));
    out[0x74] = Instruct::build_instruct(0x74, String::from("LD (HL) H"), String::from("Load H in ram[HL]"), 0, 8, Op::Ram(instruct_fn::ld_hlp_h));
    out[0x75] = Instruct::build_instruct(0x75, String::from("LD (HL) L"), String::from("Load L in ram[HL]"), 0, 8, Op::Ram(instruct_fn::ld_hlp_l));
    out[0x36] = Instruct::build_instruct(0x36, String::from("LD (HL) n"), String::from("Load n in ram[HL]"), 1, 12, Op::RamU8(instruct_fn::ld_hlp_u8));
    out[0x77] = Instruct::build_instruct(0x77, String::from("LD (HL) A"), String::from("Load A in ram[HL]"), 0, 8, Op::Ram(instruct_fn::ld_hlp_a));
    //Load n from A
    out[0x02] = Instruct::build_instruct(0x02, String::from("LD (BC) A"), String::from("Load A in ram[BC]"), 0, 8, Op::Ram(instruct_fn::ld_bcp_a));
    out[0x12] = Instruct::build_instruct(0x12, String::from("LD (DE) A"), String::from("Load A in ram[DE]"), 0, 8, Op::Ram(instruct_fn::ld_dep_a));
    out[0xEA] = Instruct::build_instruct(0xEA, String::from("LD (nn) A"), String::from("Load A in ram[nn]"), 2, 16, Op::RamU16(instruct_fn::ld_u16p_a));
    //Load A and HRam+C
    out[0xF2] = Instruct::build_instruct(0xF2, String::from("LD A (C)"), String::from("Load ram[0xFF00 + C] in A"), 0, 8, Op::Ram(instruct_fn::ld_a_hram_c));
    out[0xE2] = Instruct::build_instruct(0xE2, String::from("LD (C) A"), String::from("Load A in ram[0xFF00 + C]"), 0, 8, Op::Ram(instruct_fn::ld_hram_c_a));
    //Load A and decrease/increase HL
    out[0x3A] = Instruct::build_instruct(0x3A, String::from("LDD A (HL)"), String::from("Load ram[HL] in A, HL--"), 0, 8, Op::Ram(instruct_fn::ldd_a_hlp));
    out[0x32] = Instruct::build_instruct(0x32, String::from("LDD (HL) A"), String::from("Load A in ram[HL], HL--"), 0, 8, Op::Ram(instruct_fn::ldd_hlp_a));
    out[0x2A] = Instruct::build_instruct(0x2A, String::from("LDI A (HL)"), String::from("Load ram[HL] in A, HL++"), 0, 8, Op::Ram(instruct_fn::ldi_a_hlp));
    out[0x22] = Instruct::build_instruct(0x22, String::from("LDI (HL) A"), String::from("Load A in ram[HL], HL++"), 0, 8, Op::Ram(instruct_fn::ldi_hlp_a));
    //Load A and HRam + n
    out[0xF0] = Instruct::build_instruct(0xF0, String::from("LDH A (n)"), String::from("Load ram[0xff00+n] in A"), 1, 12, Op::RamU8(instruct_fn::ldh_a_u8));
    out[0xE0] = Instruct::build_instruct(0xE0, String::from("LDH (n) A"), String::from("Load A in ram[0xff00+n]"), 1, 12, Op::RamU8(instruct_fn::ldh_u8_a));
    //16 bits direct loads
    out[0x01] = Instruct::build_instruct(0x01, String::from("LD BC nn"), String::from("Load nn in BC"), 2, 12, Op::U16(instruct_fn::ld_bc_u16));
    out[0x11] = Instruct::build_instruct(0x11, String::from("LD DE nn"), String::from("Load nn in DE"), 2, 12, Op::U16(instruct_fn::ld_de_u16));
    out[0x21] = Instruct::build_instruct(0x21, String::from("LD HL nn"), String::from("Load nn in HL"), 2, 12, Op::U16(instruct_fn::ld_hl_u16));
    out[0x31] = Instruct::build_instruct(0x31, String::from("LD SP nn"), String::from("Load nn in SP"), 2, 12, Op::U16(instruct_fn::ld_sp_u16));
    //SP related loads
    out[0xF9] = Instruct::build_instruct(0xF9, String::from("LD SP HL"), String::from("Load HL in SP"), 0, 8, Op::No(instruct_fn::ld_sp_hl));
    out[0xF8] = Instruct::build_instruct(0xF8, String::from("LDHL SP n"), String::from("Load SP+n in HL"), 1, 12, Op::U8(instruct_fn::ld_hl_sp_i8));
    out[0x08] = Instruct::build_instruct(0x08, String::from("LD (nn) SP"), String::from("Load SP in ram[nn]"), 2, 20, Op::RamU16(instruct_fn::ld_u16_sp));
    //SP related PUSH
    out[0xF5] = Instruct::build_instruct(0xF5, String::from("PUSH AF"), String::from("Push AF in stack, SP--*2"), 0, 16, Op::Ram(instruct_fn::push_af));
    out[0xC5] = Instruct::build_instruct(0xC5, String::from("PUSH BC"), String::from("Push BC in stack, SP--*2"), 0, 16, Op::Ram(instruct_fn::push_bc));
    out[0xD5] = Instruct::build_instruct(0xD5, String::from("PUSH DE"), String::from("Push DE in stack, SP--*2"), 0, 16, Op::Ram(instruct_fn::push_de));
    out[0xE5] = Instruct::build_instruct(0xE5, String::from("PUSH HL"), String::from("Push HL in stack, SP--*2"), 0, 16, Op::Ram(instruct_fn::push_hl));
    //SP related POP
    out[0xF1] = Instruct::build_instruct(0xF1, String::from("POP AF"), String::from("Pop from stack in AF , SP++*2"), 0, 12, Op::Ram(instruct_fn::pop_af));
    out[0xC1] = Instruct::build_instruct(0xC1, String::from("POP BC"), String::from("Pop from stack in BC , SP++*2"), 0, 12, Op::Ram(instruct_fn::pop_bc));
    out[0xD1] = Instruct::build_instruct(0xD1, String::from("POP DE"), String::from("Pop from stack in DE , SP++*2"), 0, 12, Op::Ram(instruct_fn::pop_de));
    out[0xE1] = Instruct::build_instruct(0xE1, String::from("POP HL"), String::from("Pop from stack in HL , SP++*2"), 0, 12, Op::Ram(instruct_fn::pop_hl));
    //Add n to A
    out[0x87] = Instruct::build_instruct(0x87, String::from("ADD A A"), String::from("Add A to A"), 0, 4, Op::No(instruct_fn::add_a));
    out[0x80] = Instruct::build_instruct(0x80, String::from("ADD A B"), String::from("Add B to A"), 0, 4, Op::No(instruct_fn::add_b));
    out[0x81] = Instruct::build_instruct(0x81, String::from("ADD A C"), String::from("Add C to A"), 0, 4, Op::No(instruct_fn::add_c));
    out[0x82] = Instruct::build_instruct(0x82, String::from("ADD A D "), String::from("Add D to A"), 0, 4, Op::No(instruct_fn::add_d));
    out[0x83] = Instruct::build_instruct(0x83, String::from("ADD A E"), String::from("Add E to A"), 0, 4, Op::No(instruct_fn::add_e));
    out[0x84] = Instruct::build_instruct(0x84, String::from("ADD A H"), String::from("Add H to A"), 0, 4, Op::No(instruct_fn::add_h));
    out[0x85] = Instruct::build_instruct(0x85, String::from("ADD A L"), String::from("Add L to A"), 0, 4, Op::No(instruct_fn::add_l));
    out[0x86] = Instruct::build_instruct(0x86, String::from("ADD A (HL)"), String::from("Add ram[HL] to A"), 0, 8, Op::Ram(instruct_fn::add_hlp));
    out[0xC6] = Instruct::build_instruct(0xC6, String::from("ADD A n"), String::from("Add n to A"), 1, 8, Op::U8(instruct_fn::add_u8));
    //Add n + carry flag to A
    out[0x8F] = Instruct::build_instruct(0x8F, String::from("ADC A A"), String::from("Add A + Cflag to A"), 0, 4, Op::No(instruct_fn::adc_a));
    out[0x88] = Instruct::build_instruct(0x88, String::from("ADC A B"), String::from("Add B + Cflag to A"), 0, 4, Op::No(instruct_fn::adc_b));
    out[0x89] = Instruct::build_instruct(0x89, String::from("ADC A C"), String::from("Add C + Cflag to A"), 0, 4, Op::No(instruct_fn::adc_c));
    out[0x8A] = Instruct::build_instruct(0x8A, String::from("ADC A D"), String::from("Add D + Cflag to A"), 0, 4, Op::No(instruct_fn::adc_d));
    out[0x8B] = Instruct::build_instruct(0x8B, String::from("ADC A E"), String::from("Add E + Cflag to A"), 0, 4, Op::No(instruct_fn::adc_e));
    out[0x8C] = Instruct::build_instruct(0x8C, String::from("ADC A H"), String::from("Add H + Cflag to A"), 0, 4, Op::No(instruct_fn::adc_h));
    out[0x8D] = Instruct::build_instruct(0x8D, String::from("ADC A L"), String::from("Add L + Cflag to A"), 0, 4, Op::No(instruct_fn::adc_l));
    out[0x8E] = Instruct::build_instruct(0x8E, String::from("ADC A (HL)"), String::from("Add ram[HL] + Cflag to A"), 0, 8, Op::Ram(instruct_fn::adc_hlp));
    out[0xCE] = Instruct::build_instruct(0xCE, String::from("ADC A n"), String::from("Add n + Cflag to A"), 1, 8, Op::U8(instruct_fn::adc_u8));
    //Sub from A
    out[0x97] = Instruct::build_instruct(0x97, String::from("SUB A"), String::from("Sub A from A"), 0, 4, Op::No(instruct_fn::sub_a));
    out[0x90] = Instruct::build_instruct(0x90, String::from("SUB B"), String::from("Sub B from A"), 0, 4, Op::No(instruct_fn::sub_b));
    out[0x91] = Instruct::build_instruct(0x91, String::from("SUB C"), String::from("Sub C from A"), 0, 4, Op::No(instruct_fn::sub_c));
    out[0x92] = Instruct::build_instruct(0x92, String::from("SUB D"), String::from("Sub D from A"), 0, 4, Op::No(instruct_fn::sub_d));
    out[0x93] = Instruct::build_instruct(0x93, String::from("SUB E"), String::from("Sub E from A"), 0, 4, Op::No(instruct_fn::sub_e));
    out[0x94] = Instruct::build_instruct(0x94, String::from("SUB H"), String::from("Sub H from A"), 0, 4, Op::No(instruct_fn::sub_h));
    out[0x95] = Instruct::build_instruct(0x95, String::from("SUB L"), String::from("Sub L from A"), 0, 4, Op::No(instruct_fn::sub_l));
    out[0x96] = Instruct::build_instruct(0x96, String::from("SUB (HL)"), String::from("Sub ram[HL] from A"), 0, 8, Op::Ram(instruct_fn::sub_hlp));
    out[0xD6] = Instruct::build_instruct(0xD6, String::from("SUB n"), String::from("Sub n from A"), 1, 8, Op::U8(instruct_fn::sub_u8));
    //Sub n + carry flag from A
    out[0x9F] = Instruct::build_instruct(0x9F, String::from("SBC A "), String::from("Sub A + Cflag from A"), 0, 4, Op::No(instruct_fn::sbc_a));
    out[0x98] = Instruct::build_instruct(0x98, String::from("SBC B "), String::from("Sub B + Cflag from A"), 0, 4, Op::No(instruct_fn::sbc_b));
    out[0x99] = Instruct::build_instruct(0x99, String::from("SBC C "), String::from("Sub C + Cflag from A"), 0, 4, Op::No(instruct_fn::sbc_c));
    out[0x9A] = Instruct::build_instruct(0x9A, String::from("SBC D "), String::from("Sub D + Cflag from A"), 0, 4, Op::No(instruct_fn::sbc_d));
    out[0x9B] = Instruct::build_instruct(0x9B, String::from("SBC E "), String::from("Sub E + Cflag from A"), 0, 4, Op::No(instruct_fn::sbc_e));
    out[0x9C] = Instruct::build_instruct(0x9C, String::from("SBC H "), String::from("Sub H + Cflag from A"), 0, 4, Op::No(instruct_fn::sbc_h));
    out[0x9D] = Instruct::build_instruct(0x9D, String::from("SBC L "), String::from("Sub L + Cflag from A"), 0, 4, Op::No(instruct_fn::sbc_l));
    out[0x9E] = Instruct::build_instruct(0x9E, String::from("SBC (HL) "), String::from("Sub ram[HL] + Cflag from A"), 0, 8, Op::Ram(instruct_fn::sbc_hlp));
    out[0xCE] = Instruct::build_instruct(0xDE, String::from("ADC A n"), String::from("Sub n + Cflag from A"), 1, 8, Op::U8(instruct_fn::sbc_u8));
    //and(H is SET)
    out[0xA7] = Instruct::build_instruct(0xA7, String::from("AND A"), String::from("Store A & A in A"), 0, 4, Op::No(instruct_fn::and_a));
    out[0xA0] = Instruct::build_instruct(0xA0, String::from("AND B"), String::from("Store B & A in A"), 0, 4, Op::No(instruct_fn::and_b));
    out[0xA1] = Instruct::build_instruct(0xA1, String::from("AND C"), String::from("Store C & A in A"), 0, 4, Op::No(instruct_fn::and_c));
    out[0xA2] = Instruct::build_instruct(0xA2, String::from("AND D"), String::from("Store D & A in A"), 0, 4, Op::No(instruct_fn::and_d));
    out[0xA3] = Instruct::build_instruct(0xA3, String::from("AND E"), String::from("Store E & A in A"), 0, 4, Op::No(instruct_fn::and_e));
    out[0xA4] = Instruct::build_instruct(0xA4, String::from("AND H"), String::from("Store H & A in A"), 0, 4, Op::No(instruct_fn::and_h));
    out[0xA5] = Instruct::build_instruct(0xA5, String::from("AND L"), String::from("Store L & A in A"), 0, 4, Op::No(instruct_fn::and_l));
    out[0xA6] = Instruct::build_instruct(0xA6, String::from("AND (HL)"), String::from("Store ram[HL] & A in A"), 0, 8, Op::Ram(instruct_fn::and_hlp));
    out[0xE6] = Instruct::build_instruct(0xE6, String::from("AND n"), String::from("Store n & A in A"), 1, 8, Op::U8(instruct_fn::and_u8));
    //OR
    out[0xB7] = Instruct::build_instruct(0xB7, String::from("OR A"), String::from("Store A | A in A"), 0, 4, Op::No(instruct_fn::or_a));
    out[0xB0] = Instruct::build_instruct(0xB0, String::from("OR B"), String::from("Store B | A in A"), 0, 4, Op::No(instruct_fn::or_b));
    out[0xB1] = Instruct::build_instruct(0xB1, String::from("OR C"), String::from("Store C | A in A"), 0, 4, Op::No(instruct_fn::or_c));
    out[0xB2] = Instruct::build_instruct(0xB2, String::from("OR D"), String::from("Store D | A in A"), 0, 4, Op::No(instruct_fn::or_d));
    out[0xB3] = Instruct::build_instruct(0xB3, String::from("OR E"), String::from("Store E | A in A"), 0, 4, Op::No(instruct_fn::or_e));
    out[0xB4] = Instruct::build_instruct(0xB4, String::from("OR H"), String::from("Store H | A in A"), 0, 4, Op::No(instruct_fn::or_h));
    out[0xB5] = Instruct::build_instruct(0xB5, String::from("OR L"), String::from("Store L | A in A"), 0, 4, Op::No(instruct_fn::or_l));
    out[0xB6] = Instruct::build_instruct(0xB6, String::from("OR (HL)"), String::from("Store ram[HL] | A in A"), 0, 8, Op::Ram(instruct_fn::or_hlp));
    out[0xF6] = Instruct::build_instruct(0xF6, String::from("OR n"), String::from("Store n | A in A"), 1, 8, Op::U8(instruct_fn::or_u8));
    //XOR(^ = XOR)
    out[0xAF] = Instruct::build_instruct(0xAF, String::from("XOR A"), String::from("Store A ^ A in A"), 0, 4, Op::No(instruct_fn::xor_a));
    out[0xA8] = Instruct::build_instruct(0xA8, String::from("XOR B"), String::from("Store B ^ A in A"), 0, 4, Op::No(instruct_fn::xor_b));
    out[0xA9] = Instruct::build_instruct(0xA9, String::from("XOR C"), String::from("Store C ^ A in A"), 0, 4, Op::No(instruct_fn::xor_c));
    out[0xAA] = Instruct::build_instruct(0xAA, String::from("XOR D"), String::from("Store D ^ A in A"), 0, 4, Op::No(instruct_fn::xor_d));
    out[0xAB] = Instruct::build_instruct(0xAB, String::from("XOR E"), String::from("Store E ^ A in A"), 0, 4, Op::No(instruct_fn::xor_e));
    out[0xAC] = Instruct::build_instruct(0xAC, String::from("XOR H"), String::from("Store H ^ A in A"), 0, 4, Op::No(instruct_fn::xor_h));
    out[0xAD] = Instruct::build_instruct(0xAD, String::from("XOR L"), String::from("Store L ^ A in A"), 0, 4, Op::No(instruct_fn::xor_l));
    out[0xAE] = Instruct::build_instruct(0xAE, String::from("XOR (HL)"), String::from("Store ram[HL] ^ A in A"), 0, 8, Op::Ram(instruct_fn::xor_hlp));
    out[0xEE] = Instruct::build_instruct(0xEE, String::from("XOR n"), String::from("Store n ^ A in A"), 1, 8, Op::U8(instruct_fn::xor_u8));
    //CMP(Z if A=n, C if A<n)
    out[0xBF] = Instruct::build_instruct(0xBF, String::from("CP A"), String::from("Compare A and A"), 0, 4, Op::No(instruct_fn::cp_a));
    out[0xB8] = Instruct::build_instruct(0xB8, String::from("CP B"), String::from("Compare B and A"), 0, 4, Op::No(instruct_fn::cp_b));
    out[0xB9] = Instruct::build_instruct(0xB9, String::from("CP C"), String::from("Compare C and A"), 0, 4, Op::No(instruct_fn::cp_c));
    out[0xBA] = Instruct::build_instruct(0xBA, String::from("CP D"), String::from("Compare D and A"), 0, 4, Op::No(instruct_fn::cp_d));
    out[0xBB] = Instruct::build_instruct(0xBB, String::from("CP E"), String::from("Compare E and A"), 0, 4, Op::No(instruct_fn::cp_e));
    out[0xBC] = Instruct::build_instruct(0xBC, String::from("CP H"), String::from("Compare H and A"), 0, 4, Op::No(instruct_fn::cp_h));
    out[0xBD] = Instruct::build_instruct(0xBD, String::from("CP L"), String::from("Compare L and A"), 0, 4, Op::No(instruct_fn::cp_l));
    out[0xBE] = Instruct::build_instruct(0xBE, String::from("CP (HL)"), String::from("Compare ram[HL] and A"), 0, 8, Op::Ram(instruct_fn::cp_hlp));
    out[0xFE] = Instruct::build_instruct(0xFE, String::from("CP n"), String::from("Compare n and A"), 1, 8, Op::U8(instruct_fn::cp_u8));
    //Inc
    out[0x3C] = Instruct::build_instruct(0x3C, String::from("INC A"), String::from("Increment A"), 0, 4, Op::No(instruct_fn::inc_a));
    out[0x04] = Instruct::build_instruct(0x04, String::from("INC B"), String::from("Increment B"), 0, 4, Op::No(instruct_fn::inc_b));
    out[0x0C] = Instruct::build_instruct(0x0C, String::from("INC C"), String::from("Increment C"), 0, 4, Op::No(instruct_fn::inc_c));
    out[0x14] = Instruct::build_instruct(0x14, String::from("INC D"), String::from("Increment D"), 0, 4, Op::No(instruct_fn::inc_d));
    out[0x1C] = Instruct::build_instruct(0x1C, String::from("INC E"), String::from("Increment E"), 0, 4, Op::No(instruct_fn::inc_e));
    out[0x24] = Instruct::build_instruct(0x24, String::from("INC H"), String::from("Increment H"), 0, 4, Op::No(instruct_fn::inc_h));
    out[0x2C] = Instruct::build_instruct(0x2C, String::from("INC L"), String::from("Increment L"), 0, 4, Op::No(instruct_fn::inc_l));
    out[0x34] = Instruct::build_instruct(0x34, String::from("INC (HL)"), String::from("Increment ram[HL]"), 0, 12, Op::Ram(instruct_fn::inc_hlp));
    //Dec
    out[0x3D] = Instruct::build_instruct(0x3D, String::from("DEC A"), String::from("Decrement A"), 0, 4, Op::No(instruct_fn::dec_a));
    out[0x05] = Instruct::build_instruct(0x05, String::from("DEC B"), String::from("Decrement B"), 0, 4, Op::No(instruct_fn::dec_b));
    out[0x0D] = Instruct::build_instruct(0x0D, String::from("DEC C"), String::from("Decrement C"), 0, 4, Op::No(instruct_fn::dec_c));
    out[0x15] = Instruct::build_instruct(0x15, String::from("DEC D"), String::from("Decrement D"), 0, 4, Op::No(instruct_fn::dec_d));
    out[0x1D] = Instruct::build_instruct(0x1D, String::from("DEC E"), String::from("Decrement E"), 0, 4, Op::No(instruct_fn::dec_e));
    out[0x25] = Instruct::build_instruct(0x25, String::from("DEC H"), String::from("Decrement H"), 0, 4, Op::No(instruct_fn::dec_h));
    out[0x2D] = Instruct::build_instruct(0x2D, String::from("DEC L"), String::from("Decrement L"), 0, 4, Op::No(instruct_fn::dec_l));
    out[0x35] = Instruct::build_instruct(0x35, String::from("DEC (HL)"), String::from("Decrement ram[HL]"), 0, 12, Op::Ram(instruct_fn::dec_hlp));
    //ADD 16 bits
    out[0x09] = Instruct::build_instruct(0x09, String::from("ADD HL BC"), String::from("Add BC to HL"), 0, 8, Op::No(instruct_fn::add_hl_bc));
    out[0x19] = Instruct::build_instruct(0x19, String::from("ADD HL DE "), String::from("Add DE to HL"), 0, 8, Op::No(instruct_fn::add_hl_de));
    out[0x29] = Instruct::build_instruct(0x29, String::from("ADD HL HL"), String::from("Add HL to HL"), 0, 8, Op::No(instruct_fn::add_hl_hl));
    out[0x39] = Instruct::build_instruct(0x39, String::from("ADD HL SP"), String::from("Add SP to HL"), 0, 8, Op::No(instruct_fn::add_hl_sp));
    //Add Sp
    out[0xE8] = Instruct::build_instruct(0xE8, String::from("ADD SP n /!\\"), String::from("Add n to SP"), 1, 16, Op::No(instruct_fn::nop)); //TODO use add_16?
    //INC 16 bits
    out[0x03] = Instruct::build_instruct(0x03, String::from("INC BC"), String::from("Increment BC"), 0, 8, Op::No(instruct_fn::inc_bc));
    out[0x13] = Instruct::build_instruct(0x13, String::from("INC DE"), String::from("Increment DE"), 0, 8, Op::No(instruct_fn::inc_de));
    out[0x23] = Instruct::build_instruct(0x23, String::from("INC HL"), String::from("Increment HL"), 0, 8, Op::No(instruct_fn::inc_hl));
    out[0x33] = Instruct::build_instruct(0x33, String::from("INC SP"), String::from("Increment SP"), 0, 8, Op::No(instruct_fn::inc_sp));
    //DEC 16 bits
    out[0x0B] = Instruct::build_instruct(0x0B, String::from("DEC BC"), String::from("Decrement BC"), 0, 8, Op::No(instruct_fn::dec_bc));
    out[0x1B] = Instruct::build_instruct(0x1B, String::from("DEC DE"), String::from("Decrement DE"), 0, 8, Op::No(instruct_fn::dec_de));
    out[0x2B] = Instruct::build_instruct(0x2B, String::from("DEC HL"), String::from("Decrement HL"), 0, 8, Op::No(instruct_fn::dec_hl));
    out[0x3B] = Instruct::build_instruct(0x3B, String::from("DEC SP"), String::from("Decrement SP"), 0, 8, Op::No(instruct_fn::dec_sp));
    //SWAP
    //out[0x37] = Instruct::build_instruct(0x37, String::from("SWAP A /!\\"), String::from("Swap upper and lower nibble of A"), 0, 8, Op::No(instruct_fn::nop));
    //out[0x30] = Instruct::build_instruct(0x30, String::from("SWAP B /!\\"), String::from("Swap upper and lower nibble of B"), 0, 8, Op::No(instruct_fn::nop));
    //out[0x31] = Instruct::build_instruct(0x31, String::from("SWAP C /!\\"), String::from("Swap upper and lower nibble of C"), 0, 8, Op::No(instruct_fn::nop));
    //out[0x32] = Instruct::build_instruct(0x32, String::from("SWAP D /!\\"), String::from("Swap upper and lower nibble of D"), 0, 8, Op::No(instruct_fn::nop));
    //out[0x33] = Instruct::build_instruct(0x33, String::from("SWAP E /!\\"), String::from("Swap upper and lower nibble of E"), 0, 8, Op::No(instruct_fn::nop));
    //out[0x34] = Instruct::build_instruct(0x34, String::from("SWAP H /!\\"), String::from("Swap upper and lower nibble of H"), 0, 8, Op::No(instruct_fn::nop));
    //out[0x35] = Instruct::build_instruct(0x35, String::from("SWAP L /!\\"), String::from("Swap upper and lower nibble of L"), 0, 8, Op::No(instruct_fn::nop));
    //out[0x36] = Instruct::build_instruct(0x36, String::from("SWAP (HL) /!\\"), String::from("Swap upper and lower nibble of ram[HL]"), 0, 16, Op::No(instruct_fn::nop));
    //MISC
    out[0x27] = Instruct::build_instruct(0x27, String::from("DAA"), String::from("Adjust A to obtain a correct BCD"), 0, 4, Op::No(instruct_fn::daa));
    out[0x2F] = Instruct::build_instruct(0x2F, String::from("CPL"), String::from("Flip all bits of A"), 0, 4, Op::No(instruct_fn::cpl));
    out[0x3F] = Instruct::build_instruct(0x3F, String::from("CCF /!\\"), String::from("Flip C flag"), 0, 4, Op::No(instruct_fn::nop));
    out[0x37] = Instruct::build_instruct(0x37, String::from("SCF /!\\"), String::from("Set C flag, reset N and H"), 0, 4, Op::No(instruct_fn::nop));
    out[0x76] = Instruct::build_instruct(0x76, String::from("HALT /!\\"), String::from("Stop CPU until interrupt is received"), 0, 4, Op::No(instruct_fn::nop));
    out[0x10] = Instruct::build_instruct(0x10, String::from("STOP /!\\"), String::from("Stop CPU and LCD until button pressed"), 0, 4, Op::No(instruct_fn::nop));
    out[0xF3] = Instruct::build_instruct(0xF3, String::from("DI"), String::from("Disable interrupts"), 0, 4, Op::No(instruct_fn::di));
    out[0xFB] = Instruct::build_instruct(0xFB, String::from("EI"), String::from("Enable interrupts"), 0, 4, Op::No(instruct_fn::ei));
    //Rotates
    out[0x07] = Instruct::build_instruct(0x07, String::from("RLCA"), String::from("Rotate A left, old bit 7 to C flag."), 0, 4, Op::No(instruct_fn::rlca));
    out[0x17] = Instruct::build_instruct(0x17, String::from("RLA /!\\"), String::from("Rotate A left through C flag."), 0, 4, Op::No(instruct_fn::nop));
    out[0x0F] = Instruct::build_instruct(0x0F, String::from("RCCA /!\\"), String::from("Rotate A right, old bit 0 to C flag."), 0, 4, Op::No(instruct_fn::nop));
    out[0x1F] = Instruct::build_instruct(0x1F, String::from("RCA /!\\"), String::from("Rotate A right through C flag."), 0, 4, Op::No(instruct_fn::nop));
    out[0xCB] = Instruct::build_instruct(0xCB, String::from("PREFIX 5/256/!\\"), String::from("My personal favourite <3"), 1, 12, Op::RamU8(instruct_fn::prefix));//TODO TIMING
    //(Synthé keyboard) MIGHT AS WELL JUMP (JUMP)
    out[0xC3] = Instruct::build_instruct(0xC3, String::from("JP nn"), String::from("Jump to nn"), 2, 16, Op::U16(instruct_fn::jp_u16));
    out[0xC2] = Instruct::build_instruct(0xC2, String::from("JP NZ nn"), String::from("Jump to nn if Z=0"), 2, 12, Op::U16(instruct_fn::jp_nz_u16));
    out[0xCA] = Instruct::build_instruct(0xCA, String::from("JP Z nn"), String::from("Jump to nn if Z=1"), 2, 12, Op::U16(instruct_fn::jp_z_u16));
    out[0xD2] = Instruct::build_instruct(0xD2, String::from("JP NC nn"), String::from("Jump to nn if C=0"), 2, 12, Op::U16(instruct_fn::jp_nc_u16));
    out[0xDA] = Instruct::build_instruct(0xDA, String::from("JP C nn"), String::from("Jump to nn if C=1"), 2, 12, Op::U16(instruct_fn::jp_c_u16));
    out[0xE9] = Instruct::build_instruct(0xE9, String::from("JP (HL)"), String::from("Jump to (HL)"), 0, 4, Op::No(instruct_fn::jp_hl));
    out[0x18] = Instruct::build_instruct(0x18, String::from("JR i"), String::from("Add i to PC"), 1, 12, Op::U8(instruct_fn::jpr));
    out[0x20] = Instruct::build_instruct(0x20, String::from("JR NZ i"), String::from("Add i to PC if Z=0"), 1, 8, Op::U8(instruct_fn::jpr_nz));
    out[0x28] = Instruct::build_instruct(0x28, String::from("JR Z i"), String::from("Add i to PC if Z=1"), 1, 8, Op::U8(instruct_fn::jpr_z));
    out[0x30] = Instruct::build_instruct(0x30, String::from("JR NC i"), String::from("Add i to PC if C=0"), 1, 8, Op::U8(instruct_fn::jpr_nc));
    out[0x38] = Instruct::build_instruct(0x38, String::from("JR C i"), String::from("Add i to PC if C=1"), 1, 8, Op::U8(instruct_fn::jpr_c));
    //Calls
    out[0xCD] = Instruct::build_instruct(0xCD, String::from("CALL nn"), String::from("Push next adress onto stack and jump to nn"), 2, 24, Op::RamU16(instruct_fn::call_u16));
    out[0xC4] = Instruct::build_instruct(0xC4, String::from("CALL NZ nn"), String::from("Push next adress onto stack and jump to nn if Z=0"), 2, 12, Op::RamU16(instruct_fn::call_nz_u16));
    out[0xCC] = Instruct::build_instruct(0xCC, String::from("CALL Z nn"), String::from("Push next adress onto stack and jump to nn if Z=1"), 2, 12, Op::RamU16(instruct_fn::call_z_u16));
    out[0xD4] = Instruct::build_instruct(0xD4, String::from("CALL NC nn"), String::from("Push next adress onto stack and jump to nn if C=0"), 2, 12, Op::RamU16(instruct_fn::call_nc_u16));
    out[0xDC] = Instruct::build_instruct(0xDC, String::from("CALL C nn"), String::from("Push next adress onto stack and jump to nn if C=1"), 2, 12, Op::RamU16(instruct_fn::call_c_u16));
    //Restart
    out[0xC7] = Instruct::build_instruct(0xC7, String::from("RST 00"), String::from("Push present adress and jump to ram[0x0000]"), 0, 16, Op::Ram(instruct_fn::rst_0));
    out[0xCF] = Instruct::build_instruct(0xCF, String::from("RST 08"), String::from("Push present adress and jump to ram[0x0008]"), 0, 16, Op::Ram(instruct_fn::rst_8));
    out[0xD7] = Instruct::build_instruct(0xD7, String::from("RST 10"), String::from("Push present adress and jump to ram[0x0010]"), 0, 16, Op::Ram(instruct_fn::rst_10));
    out[0xDF] = Instruct::build_instruct(0xDF, String::from("RST 18"), String::from("Push present adress and jump to ram[0x0018]"), 0, 16, Op::Ram(instruct_fn::rst_18));
    out[0xE7] = Instruct::build_instruct(0xE7, String::from("RST 20"), String::from("Push present adress and jump to ram[0x0020]"), 0, 16, Op::Ram(instruct_fn::rst_20));
    out[0xEF] = Instruct::build_instruct(0xEF, String::from("RST 28"), String::from("Push present adress and jump to ram[0x0028]"), 0, 16, Op::Ram(instruct_fn::rst_28));
    out[0xF7] = Instruct::build_instruct(0xF7, String::from("RST 30"), String::from("Push present adress and jump to ram[0x0030]"), 0, 16, Op::Ram(instruct_fn::rst_30));
    out[0xFF] = Instruct::build_instruct(0xFF, String::from("RST 38"), String::from("Push present adress and jump to ram[0x0038]"), 0, 16, Op::Ram(instruct_fn::rst_38));
    //Returns
    out[0xC9] = Instruct::build_instruct(0xC9, String::from("RET"), String::from("Pop from stack and jump to adress"), 0, 16, Op::Ram(instruct_fn::ret));
    out[0xC0] = Instruct::build_instruct(0xC0, String::from("RET NZ"), String::from("Pop from stack and jump to adress if Z=0"), 0, 8, Op::Ram(instruct_fn::ret_nz));
    out[0xC8] = Instruct::build_instruct(0xC8, String::from("RET Z"), String::from("Pop from stack and jump to adress if Z=1"), 0, 8, Op::Ram(instruct_fn::ret_z));
    out[0xD0] = Instruct::build_instruct(0xD0, String::from("RET NC"), String::from("Pop from stack and jump to adress if C=0"), 0, 8, Op::Ram(instruct_fn::ret_nc));
    out[0xD8] = Instruct::build_instruct(0xD8, String::from("RET C"), String::from("Pop from stack and jump to adress if C=1"), 0, 8, Op::Ram(instruct_fn::ret_c));
    out[0xD9] = Instruct::build_instruct(0xD9, String::from("RETI"), String::from("Same as RET, but enable interrupts"), 0, 16, Op::Ram(instruct_fn::ret_i));

    //What's left
    return out;
}

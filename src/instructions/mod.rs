use crate::hardware;
use hardware::{Flag, RegU8, RegU16, Cpu};
use std::fmt;

mod instruct_fn;

pub struct Instruct {
    pub opcode: u8,
    pub inst: InstructType,
    pub desc: String,
    pub ticks: u8,
}

pub enum InstructType {
    Load(RegU8, RegU8), // Destination, source
    LoadPlus(RegU8, RegU8, bool), // Destination, source, increase (true) / decrease (false)
    LoadU16(RegU16, RegU16), // Destination, source
    Push(RegU16), // Source (dest is STACK)
    Pop(RegU16), // Destintation (source is STACK)
    Add(RegU8, bool), // Source, with carry (true) / without carry (false)
    Sub(RegU8, bool), // Source, with carry (true) / without carry (false)
    And(RegU8), // Source (dest is A)
    Or(RegU8), // Source (dest is A)
    Xor(RegU8), // Source (dest is A)
    Cmp(RegU8), // Source (dest is A)
    Inc(RegU8), // Register
    Dec(RegU8), // Register
    AddU16(RegU16), // Source (dest is HL)
    AddU16I8(RegU16, RegU16), // Destination, immediate signed offset (source is SP)
    IncU16(RegU16), // Register
    DecU16(RegU16), // Register
    Daa, // (register is A)
    Cpl, // (register is A)
    SetCarry(bool), // Flip flag (true) / set flag (false)
    Nop,
    ChangeMie(bool), // enable interrupts (true) / disable interrupts (false)
    Rotate(RegU8, bool, bool, bool, bool, bool), // Register, left/right, through carry, update_z, shift/rotate, keep_msb
    Swap(RegU8), // Register
    Bit(RegU8, u8), // Register, bit position (0-7)
    SetBit(RegU8, u8), // Register, bit position (0-7)
    ResetBit(RegU8, u8), // Register, bit position (0-7)
    Jump(RegU16, bool, Option<Flag>, bool), // Address, call / jump, unconditionnal / flag dependant, jump if set / if reset
    Reset(u16), // N-th byte of the first page to jump to (i.e. 2 = 0x0010)
    Ret(Option<Flag>, bool, bool), // Unconditionnal / flag dependant, jump if set / if reset, enable interrupts / do nothing
}

use InstructType::*;

impl InstructType {
    pub fn exec(&self, cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
        match self {
            Load(dest, src) => instruct_fn::load(cpu, ram, dest, src),
            LoadPlus(dest, src, inc) => instruct_fn::load_plus(cpu, ram, dest, src, *inc),
            LoadU16(dest, src) => instruct_fn::load_u16(cpu, ram, dest, src),
            Push(src) => instruct_fn::push(cpu, ram, src),
            Pop(dest) => instruct_fn::pop(cpu, ram, dest),
            Add(src, carry) => instruct_fn::add(cpu, ram, src, *carry),
            Sub(src, carry) => instruct_fn::sub(cpu, ram, src, *carry),
            And(src) => instruct_fn::and(cpu, ram, src),
            Or(src) => instruct_fn::or(cpu, ram, src),
            Xor(src) => instruct_fn::xor(cpu, ram, src),
            Cmp(src) => { instruct_fn::cmp(cpu, ram, src, false); },
            Inc(reg) => instruct_fn::inc(cpu, ram, reg),
            Dec(reg) => instruct_fn::dec(cpu, ram, reg),
            AddU16(src) => instruct_fn::add_u16(cpu, ram, src),
            AddU16I8(dest, offset) => instruct_fn::add_u16_i8(cpu, ram, dest, offset),
            IncU16(reg) => instruct_fn::inc_u16(cpu, ram, reg),
            DecU16(reg) => instruct_fn::dec_u16(cpu, ram, reg),
            Daa => instruct_fn::daa(cpu, ram),
            Cpl => instruct_fn::cpl(cpu, ram),
            SetCarry(flip) => instruct_fn::set_carry(cpu, ram, *flip),
            Nop => {},
            ChangeMie(enable) => instruct_fn::change_mie(cpu, ram, *enable),
            Rotate(reg, left, through_carry, update_z, shift, keep_msb) => 
                instruct_fn::rotate(cpu, ram, reg, *left, *through_carry, *update_z, *shift, *keep_msb),
            Swap(reg) => instruct_fn::swap(cpu, ram, reg),
            Bit(reg, bit_pos) => instruct_fn::bit(cpu, ram, reg, *bit_pos),
            SetBit(reg, bit_pos) => instruct_fn::set_bit(cpu, ram, reg, *bit_pos),
            ResetBit(reg, bit_pos) => instruct_fn::reset_bit(cpu, ram, reg, *bit_pos),
            Jump(addr, is_call, flag, is_set) => instruct_fn::jump(cpu, ram, addr, *is_call, flag.as_ref(), *is_set),
            Reset(nth_byte) => instruct_fn::reset(cpu, ram, *nth_byte),
            Ret(flag, is_set, i_enable) => instruct_fn::ret(cpu, ram, flag.as_ref(), *is_set, *i_enable),
        }
    }
}

impl fmt::Display for InstructType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Load(dest, src) => write!(f, "LD {} {}", dest, src),
            LoadPlus(dest, src, inc) => write!(f, "LD{} {} {}", if *inc { "I" } else { "D" }, dest, src),
            LoadU16(dest, src) => write!(f, "LD {} {}", dest, src),
            Push(src) => write!(f, "PUSH {}", src),
            Pop(dest) => write!(f, "POP {}", dest),
            Add(src, carry) => write!(f, "{} A {}", if *carry { "ADD" } else { "ADC" }, src),
            Sub(src, carry) => write!(f, "{} A {}", if *carry { "SUB" } else { "SBC" }, src),
            And(src) => write!(f, "AND {}", src),
            Or(src) => write!(f, "OR {}", src),
            Xor(src) => write!(f, "XOR {}", src),
            Cmp(src) => write!(f, "CP {}", src),
            Inc(reg) => write!(f, "INC {}", reg),
            Dec(reg) => write!(f, "DEC {}", reg),
            AddU16(src) => write!(f, "ADD HL {}", src),
            AddU16I8(dest, _) => write!(f, "{}", if dest == &RegU16::SP { "ADD SP i8" } else { "LD HL SP+i8" }),
            IncU16(reg) => write!(f, "INC {}", reg),
            DecU16(reg) => write!(f, "DEC {}", reg),
            Daa => write!(f, "DAA"),
            Cpl => write!(f, "CPL"),
            SetCarry(flip) => write!(f, "{}", if *flip { "CCF" } else { "SCF" }),
            Nop => write!(f, "NOP"),
            ChangeMie(enable) => write!(f, "{}", if *enable { "EI" } else { "DI" }),
            Rotate(reg, left, through_carry, update_z, shift, keep_msb) => write!(f, "{}{}{}{}{}",
                if *shift { "S" } else { "R" }, if *left { "L" } else { "R" },
                if *shift { if !*left && !*keep_msb { "L" } else { "A" } } else { if *through_carry { "" } else { "C" } },
                if *update_z { " " } else { "" }, reg
            ),
            Swap(reg) => write!(f, "SWAP {}", reg),
            Bit(reg, bit_pos) => write!(f, "BIT {} {}", bit_pos, reg),
            SetBit(reg, bit_pos) => write!(f, "SET {} {}", bit_pos, reg),
            ResetBit(reg, bit_pos) => write!(f, "RES {} {}", bit_pos, reg),
            Jump(addr, is_call, flag, is_set) => write!(f, "{} {}{}", if *is_call { "CALL" } else { if addr == &RegU16::I8 { "JR" } else { "JP" } },
                if let Some(flag) = flag { format!("{}{} ", if *is_set { "" } else { "N" }, flag) } else { "".to_string() }, addr
            ),
            Reset(nth_byte) => write!(f, "RST {:02x}", nth_byte * 8),
            Ret(flag, is_set, i_enable) => write!(f, "RET{}{}", if *i_enable { "I" } else { "" },
                if let Some(flag) = flag { format!(" {}{}", if *is_set { "" } else { "N" }, flag) } else { "".to_string() }
            ),
        }
    }
}

impl Instruct {
    pub fn fetch(cpu: &mut Cpu, opcode: u8, followup_byte: u8) -> Instruct {
        macro_rules! inst {
            ($inst:expr, $ticks:expr, $desc:expr) => {
                Instruct {
                    opcode,
                    inst: $inst,
                    desc: String::from($desc),
                    ticks: $ticks,
                }
            };
        }

        match opcode {
            //Load A
            0x3E => inst!(Load(RegU8::A, RegU8::U8), 8, "Load n in A"),
            0x7f => inst!(Load(RegU8::A, RegU8::A), 4, "Load A in A"),
            0x78 => inst!(Load(RegU8::A, RegU8::B), 4, "Load B in A"),
            0x79 => inst!(Load(RegU8::A, RegU8::C), 4, "Load C in A"),
            0x7a => inst!(Load(RegU8::A, RegU8::D), 4, "Load D in A"),
            0x7b => inst!(Load(RegU8::A, RegU8::E), 4, "Load E in A"),
            0x7C => inst!(Load(RegU8::A, RegU8::H), 4, "Load H in A"),
            0x7D => inst!(Load(RegU8::A, RegU8::L), 4, "Load L in A"),
            0x0A => inst!(Load(RegU8::A, RegU8::RamU16(RegU16::BC)), 8, "Load ram[BC] in A"),
            0x1A => inst!(Load(RegU8::A, RegU8::RamU16(RegU16::DE)), 8, "Load ram[DE] in A"),
            0x7E => inst!(Load(RegU8::A, RegU8::RamU16(RegU16::HL)), 8, "Load ram[HL] in A"),
            0xFA => inst!(Load(RegU8::A, RegU8::RamU16(RegU16::U16)), 16, "Load ram[nn] in A"),
            //Load B
            0x06 => inst!(Load(RegU8::B, RegU8::U8), 8, "Load n in B"),
            0x40 => inst!(Load(RegU8::B, RegU8::B), 4, "Load B in B"),
            0x41 => inst!(Load(RegU8::B, RegU8::C), 4, "Load C in B"),
            0x42 => inst!(Load(RegU8::B, RegU8::D), 4, "Load D in B"),
            0x43 => inst!(Load(RegU8::B, RegU8::E), 4, "Load E in B"),
            0x44 => inst!(Load(RegU8::B, RegU8::H), 4, "Load H in B"),
            0x45 => inst!(Load(RegU8::B, RegU8::L), 4, "Load L in B"),
            0x46 => inst!(Load(RegU8::B, RegU8::RamU16(RegU16::HL)), 8, "Load ram[HL] in B"),
            0x47 => inst!(Load(RegU8::B, RegU8::A), 4, "Load A in B"),
            //Load C
            0x0e => inst!(Load(RegU8::C, RegU8::U8), 8, "Load n in C"),
            0x48 => inst!(Load(RegU8::C, RegU8::B), 4, "Load B in C"),
            0x49 => inst!(Load(RegU8::C, RegU8::C), 4, "Load C in C"),
            0x4A => inst!(Load(RegU8::C, RegU8::D), 4, "Load D in C"),
            0x4B => inst!(Load(RegU8::C, RegU8::E), 4, "Load E in C"),
            0x4C => inst!(Load(RegU8::C, RegU8::H), 4, "Load H in C"),
            0x4D => inst!(Load(RegU8::C, RegU8::L), 4, "Load L in C"),
            0x4E => inst!(Load(RegU8::C, RegU8::RamU16(RegU16::HL)), 8, "Load ram[HL] in C"),
            0x4f => inst!(Load(RegU8::C, RegU8::A), 4, "Load A in C"),
            //Load D
            0x16 => inst!(Load(RegU8::D, RegU8::U8), 8, "Load n in D"),
            0x50 => inst!(Load(RegU8::D, RegU8::B), 4, "Load B in D"),
            0x51 => inst!(Load(RegU8::D, RegU8::C), 4, "Load C in D"),
            0x52 => inst!(Load(RegU8::D, RegU8::D), 4, "Load D in D"),
            0x53 => inst!(Load(RegU8::D, RegU8::E), 4, "Load E in D"),
            0x54 => inst!(Load(RegU8::D, RegU8::H), 4, "Load H in D"),
            0x55 => inst!(Load(RegU8::D, RegU8::L), 4, "Load L in D"),
            0x56 => inst!(Load(RegU8::D, RegU8::RamU16(RegU16::HL)), 8, "Load ram[HL] in D"),
            0x57 => inst!(Load(RegU8::D, RegU8::A), 4, "Load A in D"),
            //Load E
            0x1e => inst!(Load(RegU8::E, RegU8::U8), 8, "Load n in E"),
            0x58 => inst!(Load(RegU8::E, RegU8::B), 4, "Load B in E"),
            0x59 => inst!(Load(RegU8::E, RegU8::C), 4, "Load C in E"),
            0x5a => inst!(Load(RegU8::E, RegU8::D), 4, "Load D in E"),
            0x5b => inst!(Load(RegU8::E, RegU8::E), 4, "Load E in E"),
            0x5c => inst!(Load(RegU8::E, RegU8::H), 4, "Load H in E"),
            0x5d => inst!(Load(RegU8::E, RegU8::L), 4, "Load L in E"),
            0x5e => inst!(Load(RegU8::E, RegU8::RamU16(RegU16::HL)), 8, "Load ram[HL] in E"),
            0x5F => inst!(Load(RegU8::E, RegU8::A), 4, "Load A in E"),
            //Load H
            0x26 => inst!(Load(RegU8::H, RegU8::U8), 8, "Load n in H"),
            0x60 => inst!(Load(RegU8::H, RegU8::B), 4, "Load B in H"),
            0x61 => inst!(Load(RegU8::H, RegU8::C), 4, "Load C in H"),
            0x62 => inst!(Load(RegU8::H, RegU8::D), 4, "Load D in H"),
            0x63 => inst!(Load(RegU8::H, RegU8::E), 4, "Load E in H"),
            0x64 => inst!(Load(RegU8::H, RegU8::H), 4, "Load H in H"),
            0x65 => inst!(Load(RegU8::H, RegU8::L), 4, "Load L in H"),
            0x66 => inst!(Load(RegU8::H, RegU8::RamU16(RegU16::HL)), 8, "Load ram[HL] in H"),
            0x67 => inst!(Load(RegU8::H, RegU8::A), 4, "Load A in H"),
            //Load L
            0x2e => inst!(Load(RegU8::L, RegU8::U8), 8, "Load n in L"),
            0x68 => inst!(Load(RegU8::L, RegU8::B), 4, "Load B in L"),
            0x69 => inst!(Load(RegU8::L, RegU8::C), 4, "Load C in L"),
            0x6a => inst!(Load(RegU8::L, RegU8::D), 4, "Load D in L"),
            0x6b => inst!(Load(RegU8::L, RegU8::E), 4, "Load E in L"),
            0x6c => inst!(Load(RegU8::L, RegU8::H), 4, "Load H in L"),
            0x6d => inst!(Load(RegU8::L, RegU8::L), 4, "Load L in L"),
            0x6e => inst!(Load(RegU8::L, RegU8::RamU16(RegU16::HL)), 8, "Load ram[HL] in L"),
            0x6f => inst!(Load(RegU8::L, RegU8::A), 4, "Load A in L"),
            //Load (HL)
            0x36 => inst!(Load(RegU8::RamU16(RegU16::HL), RegU8::U8), 12, "Store n in ram[HL]"),
            0x70 => inst!(Load(RegU8::RamU16(RegU16::HL), RegU8::B), 8, "Store B in ram[HL]"),
            0x71 => inst!(Load(RegU8::RamU16(RegU16::HL), RegU8::C), 8, "Store C in ram[HL]"),
            0x72 => inst!(Load(RegU8::RamU16(RegU16::HL), RegU8::D), 8, "Store D in ram[HL]"),
            0x73 => inst!(Load(RegU8::RamU16(RegU16::HL), RegU8::E), 8, "Store E in ram[HL]"),
            0x74 => inst!(Load(RegU8::RamU16(RegU16::HL), RegU8::H), 8, "Store H in ram[HL]"),
            0x75 => inst!(Load(RegU8::RamU16(RegU16::HL), RegU8::L), 8, "Store L in ram[HL]"),
            0x77 => inst!(Load(RegU8::RamU16(RegU16::HL), RegU8::A), 8, "Store A in ram[HL]"),
            //Load n from A
            0x02 => inst!(Load(RegU8::RamU16(RegU16::BC), RegU8::A), 8, "Store A in ram[BC]"),
            0x12 => inst!(Load(RegU8::RamU16(RegU16::DE), RegU8::A), 8, "Store A in ram[DE]"),
            0xEA => inst!(Load(RegU8::RamU16(RegU16::U16), RegU8::A), 16, "Store A in ram[nn]"),
            //Load A and HRam + C
            0xF2 => inst!(Load(RegU8::A, RegU8::RamU8(Box::new(RegU8::C))), 8, "Load ram[0xFF00 + C] in A"),
            0xE2 => inst!(Load(RegU8::RamU8(Box::new(RegU8::C)), RegU8::A), 8, "Store A in ram[0xFF00 + C]"),
            //Load A and decrease/increase HL
            0x3A => inst!(LoadPlus(RegU8::A, RegU8::RamU16(RegU16::HL), false), 8, "Load ram[HL] in A, HL--"),
            0x32 => inst!(LoadPlus(RegU8::RamU16(RegU16::HL), RegU8::A, false), 8, "Store A in ram[HL], HL--"),
            0x2A => inst!(LoadPlus(RegU8::A, RegU8::RamU16(RegU16::HL), true), 8, "Load ram[HL] in A, HL++"),
            0x22 => inst!(LoadPlus(RegU8::RamU16(RegU16::HL), RegU8::A, true), 8, "Store A in ram[HL], HL++"),
            //Load A and HRam + n
            0xF0 => inst!(Load(RegU8::A, RegU8::RamU8(Box::new(RegU8::U8))), 12, "Load ram[0xFF00 + n] in A"),
            0xE0 => inst!(Load(RegU8::RamU8(Box::new(RegU8::U8)), RegU8::A), 12, "Store A in ram[0xFF00 + n]"),
            //16 bits direct loads
            0x01 => inst!(LoadU16(RegU16::BC, RegU16::U16), 12, "Load nn in BC"),
            0x11 => inst!(LoadU16(RegU16::DE, RegU16::U16), 12, "Load nn in DE"),
            0x21 => inst!(LoadU16(RegU16::HL, RegU16::U16), 12, "Load nn in HL"),
            0x31 => inst!(LoadU16(RegU16::SP, RegU16::U16), 12, "Load nn in SP"),
            //SP related loads
            0xF9 => inst!(LoadU16(RegU16::SP, RegU16::HL), 8, "Load HL in SP"),
            0xF8 => inst!(AddU16I8(RegU16::HL, RegU16::I8), 12, "Load SP+i8 in HL"),
            0x08 => inst!(LoadU16(RegU16::RamU16(Box::new(RegU16::U16)), RegU16::SP), 20, "Store SP in ram[nn]"),
            //SP related PUSH
            0xF5 => inst!(Push(RegU16::AF), 16, "Push AF on stack, SP-=2"),
            0xC5 => inst!(Push(RegU16::BC), 16, "Push BC on stack, SP-=2"),
            0xD5 => inst!(Push(RegU16::DE), 16, "Push DE on stack, SP-=2"),
            0xE5 => inst!(Push(RegU16::HL), 16, "Push HL on stack, SP-=2"),
            //SP related POP
            0xF1 => inst!(Pop(RegU16::AF), 12, "Pop from stack to AF, SP+=2"),
            0xC1 => inst!(Pop(RegU16::BC), 12, "Pop from stack to BC, SP+=2"),
            0xD1 => inst!(Pop(RegU16::DE), 12, "Pop from stack to DE, SP+=2"),
            0xE1 => inst!(Pop(RegU16::HL), 12, "Pop from stack to HL, SP+=2"),
            //Add n to A
            0xC6 => inst!(Add(RegU8::U8, false), 8, "Add n to A"),
            0x87 => inst!(Add(RegU8::A, false), 4, "Add A to A"),
            0x80 => inst!(Add(RegU8::B, false), 4, "Add B to A"),
            0x81 => inst!(Add(RegU8::C, false), 4, "Add C to A"),
            0x82 => inst!(Add(RegU8::D, false), 4, "Add D to A"),
            0x83 => inst!(Add(RegU8::E, false), 4, "Add E to A"),
            0x84 => inst!(Add(RegU8::H, false), 4, "Add H to A"),
            0x85 => inst!(Add(RegU8::L, false), 4, "Add L to A"),
            0x86 => inst!(Add(RegU8::RamU16(RegU16::HL), false), 8, "Add ram[HL] to A"),
            //Add n + carry flag to A
            0xCE => inst!(Add(RegU8::U8, true), 8, "Add n + Cflag to A"),
            0x8F => inst!(Add(RegU8::A, true), 4, "Add A + Cflag to A"),
            0x88 => inst!(Add(RegU8::B, true), 4, "Add B + Cflag to A"),
            0x89 => inst!(Add(RegU8::C, true), 4, "Add C + Cflag to A"),
            0x8A => inst!(Add(RegU8::D, true), 4, "Add D + Cflag to A"),
            0x8B => inst!(Add(RegU8::E, true), 4, "Add E + Cflag to A"),
            0x8C => inst!(Add(RegU8::H, true), 4, "Add H + Cflag to A"),
            0x8D => inst!(Add(RegU8::L, true), 4, "Add L + Cflag to A"),
            0x8E => inst!(Add(RegU8::RamU16(RegU16::HL), true), 8, "Add ram[HL] + Cflag to A"),
            //Sub n from A
            0xD6 => inst!(Sub(RegU8::U8, false), 8, "Sub n from A"),
            0x97 => inst!(Sub(RegU8::A, false), 4, "Sub A from A"),
            0x90 => inst!(Sub(RegU8::B, false), 4, "Sub B from A"),
            0x91 => inst!(Sub(RegU8::C, false), 4, "Sub C from A"),
            0x92 => inst!(Sub(RegU8::D, false), 4, "Sub D from A"),
            0x93 => inst!(Sub(RegU8::E, false), 4, "Sub E from A"),
            0x94 => inst!(Sub(RegU8::H, false), 4, "Sub H from A"),
            0x95 => inst!(Sub(RegU8::L, false), 4, "Sub L from A"),
            0x96 => inst!(Sub(RegU8::RamU16(RegU16::HL), false), 8, "Sub ram[HL] from A"),
            //Sub n + carry flag from A
            0xDE => inst!(Sub(RegU8::U8, true), 8, "Sub n + Cflag from A"),
            0x98 => inst!(Sub(RegU8::A, true), 4, "Sub A + Cflag from A"),
            0x9F => inst!(Sub(RegU8::B, true), 4, "Sub B + Cflag from A"),
            0x99 => inst!(Sub(RegU8::C, true), 4, "Sub C + Cflag from A"),
            0x9A => inst!(Sub(RegU8::D, true), 4, "Sub D + Cflag from A"),
            0x9B => inst!(Sub(RegU8::E, true), 4, "Sub E + Cflag from A"),
            0x9C => inst!(Sub(RegU8::H, true), 4, "Sub H + Cflag from A"),
            0x9D => inst!(Sub(RegU8::L, true), 4, "Sub L + Cflag from A"),
            0x9E => inst!(Sub(RegU8::RamU16(RegU16::HL), true), 8, "Sub ram[HL] + Cflag from A"),
            //AND
            0xE6 => inst!(And(RegU8::U8), 8, "Store n & A in A"),
            0xA7 => inst!(And(RegU8::A), 4, "Store A & A in A"),
            0xA0 => inst!(And(RegU8::B), 4, "Store B & A in A"),
            0xA1 => inst!(And(RegU8::C), 4, "Store C & A in A"),
            0xA2 => inst!(And(RegU8::D), 4, "Store D & A in A"),
            0xA3 => inst!(And(RegU8::E), 4, "Store E & A in A"),
            0xA4 => inst!(And(RegU8::H), 4, "Store H & A in A"),
            0xA5 => inst!(And(RegU8::L), 4, "Store L & A in A"),
            0xA6 => inst!(And(RegU8::RamU16(RegU16::HL)), 8, "Store ram[HL] & A in A"),
            //OR
            0xF6 => inst!(Or(RegU8::U8), 8, "Store n | A in A"),
            0xB7 => inst!(Or(RegU8::A), 4, "Store A | A in A"),
            0xB0 => inst!(Or(RegU8::B), 4, "Store B | A in A"),
            0xB1 => inst!(Or(RegU8::C), 4, "Store C | A in A"),
            0xB2 => inst!(Or(RegU8::D), 4, "Store D | A in A"),
            0xB3 => inst!(Or(RegU8::E), 4, "Store E | A in A"),
            0xB4 => inst!(Or(RegU8::H), 4, "Store H | A in A"),
            0xB5 => inst!(Or(RegU8::L), 4, "Store L | A in A"),
            0xB6 => inst!(Or(RegU8::RamU16(RegU16::HL)), 8, "Store ram[HL] | A in A"),
            //XOR (^ = XOR)
            0xEE => inst!(Xor(RegU8::U8), 8, "Store n ^ A in A"),
            0xAF => inst!(Xor(RegU8::A), 4, "Store A ^ A in A"),
            0xA8 => inst!(Xor(RegU8::B), 4, "Store B ^ A in A"),
            0xA9 => inst!(Xor(RegU8::C), 4, "Store C ^ A in A"),
            0xAA => inst!(Xor(RegU8::D), 4, "Store D ^ A in A"),
            0xAB => inst!(Xor(RegU8::E), 4, "Store E ^ A in A"),
            0xAC => inst!(Xor(RegU8::H), 4, "Store H ^ A in A"),
            0xAD => inst!(Xor(RegU8::L), 4, "Store L ^ A in A"),
            0xAE => inst!(Xor(RegU8::RamU16(RegU16::HL)), 8, "Store ram[HL] ^ A in A"),
            //CMP (Z if A=n, C if A<n)
            0xFE => inst!(Cmp(RegU8::U8), 8, "Compare n and A"),
            0xBF => inst!(Cmp(RegU8::A), 4, "Compare A and A"),
            0xB8 => inst!(Cmp(RegU8::B), 4, "Compare B and A"),
            0xB9 => inst!(Cmp(RegU8::C), 4, "Compare C and A"),
            0xBA => inst!(Cmp(RegU8::D), 4, "Compare D and A"),
            0xBB => inst!(Cmp(RegU8::E), 4, "Compare E and A"),
            0xBC => inst!(Cmp(RegU8::H), 4, "Compare H and A"),
            0xBD => inst!(Cmp(RegU8::L), 4, "Compare L and A"),
            0xBE => inst!(Cmp(RegU8::RamU16(RegU16::HL)), 8, "Compare ram[HL] and A"),
            //Inc
            0x3C => inst!(Inc(RegU8::A), 4, "Increment A"),
            0x04 => inst!(Inc(RegU8::B), 4, "Increment B"),
            0x0C => inst!(Inc(RegU8::C), 4, "Increment C"),
            0x14 => inst!(Inc(RegU8::D), 4, "Increment D"),
            0x1C => inst!(Inc(RegU8::E), 4, "Increment E"),
            0x24 => inst!(Inc(RegU8::H), 4, "Increment H"),
            0x2C => inst!(Inc(RegU8::L), 4, "Increment L"),
            0x34 => inst!(Inc(RegU8::RamU16(RegU16::HL)), 12, "Increment ram[HL]"),
            //Dec
            0x3D => inst!(Dec(RegU8::A), 4, "Decrement A"),
            0x05 => inst!(Dec(RegU8::B), 4, "Decrement B"),
            0x0D => inst!(Dec(RegU8::C), 4, "Decrement C"),
            0x15 => inst!(Dec(RegU8::D), 4, "Decrement D"),
            0x1D => inst!(Dec(RegU8::E), 4, "Decrement E"),
            0x25 => inst!(Dec(RegU8::H), 4, "Decrement H"),
            0x2D => inst!(Dec(RegU8::L), 4, "Decrement L"),
            0x35 => inst!(Dec(RegU8::RamU16(RegU16::HL)), 12, "Decrement ram[HL]"),
            //ADD 16 bits
            0x09 => inst!(AddU16(RegU16::BC), 8, "Add BC to HL"),
            0x19 => inst!(AddU16(RegU16::DE), 8, "Add DE to HL"),
            0x29 => inst!(AddU16(RegU16::HL), 8, "Add HL to HL"),
            0x39 => inst!(AddU16(RegU16::SP), 8, "Add SP to HL"),
            //Add Sp
            0xE8 => inst!(AddU16I8(RegU16::SP, RegU16::I8), 16, "Add i8 to SP"),
            //INC 16 bits
            0x03 => inst!(IncU16(RegU16::BC), 8, "Increment BC"),
            0x13 => inst!(IncU16(RegU16::DE), 8, "Increment DE"),
            0x23 => inst!(IncU16(RegU16::HL), 8, "Increment HL"),
            0x33 => inst!(IncU16(RegU16::SP), 8, "Increment SP"),
            //DEC 16 bits
            0x0B => inst!(DecU16(RegU16::BC), 8, "Decrement BC"),
            0x1B => inst!(DecU16(RegU16::DE), 8, "Decrement DE"),
            0x2B => inst!(DecU16(RegU16::HL), 8, "Decrement HL"),
            0x3B => inst!(DecU16(RegU16::SP), 8, "Decrement SP"),
            //MISC
            0x27 => inst!(Daa, 4, "Adjust A to obtain a correct BCD"),
            0x2F => inst!(Cpl, 4, "Flip all bits of A"),
            0x3F => inst!(SetCarry(true), 4, "Flip C flag, reset N and H"),
            0x37 => inst!(SetCarry(false), 4, "Set C flag, reset N and H"),
            0x00 => inst!(Nop, 4, "Aussi inutile que les cours de GE00"),
            #[allow(unreachable_code)]
            0x76 => inst!(todo!(), 4, "/!\\ Stop CPU until interrupt is received"),
            #[allow(unreachable_code)]
            0x10 => inst!(todo!(), 4, "/!\\ Stop CPU and LCD until button pressed"),
            0xF3 => inst!(ChangeMie(false), 4, "Disable interrupts"),
            0xFB => inst!(ChangeMie(true), 4, "Enable interrupts"),
            //Rotates
            0x07 => inst!(Rotate(RegU8::A, true, false, false, false, false), 4, "Rotate A left (no Z)"),
            0x17 => inst!(Rotate(RegU8::A, true, true, false, false, false), 4, "Rotate A left through C flag (no Z)"),
            0x0F => inst!(Rotate(RegU8::A, false, false, false, false, false), 4, "Rotate A right (no Z)"),
            0x1F => inst!(Rotate(RegU8::A, false, true, false, false, false), 4, "Rotate A right through C flag (no Z)"),
            //Prefixed, increment PC to account for extra opcode
            0xCB => { cpu.pc = cpu.pc.wrapping_add(1); match followup_byte {
                //SWAP
                0x37 => inst!(Swap(RegU8::A), 8,"Swap upper and lower nibble of A"),
                0x30 => inst!(Swap(RegU8::B), 8,"Swap upper and lower nibble of B"),
                0x31 => inst!(Swap(RegU8::C), 8,"Swap upper and lower nibble of C"),
                0x32 => inst!(Swap(RegU8::D), 8,"Swap upper and lower nibble of D"),
                0x33 => inst!(Swap(RegU8::E), 8,"Swap upper and lower nibble of E"),
                0x34 => inst!(Swap(RegU8::H), 8,"Swap upper and lower nibble of H"),
                0x35 => inst!(Swap(RegU8::L), 8,"Swap upper and lower nibble of L"),
                0x36 => inst!(Swap(RegU8::RamU16(RegU16::HL)), 16,"Swap upper and lower nibble of ram[HL]"),
                //Rotates
                0x07 => inst!(Rotate(RegU8::A, true, false, true, false, false), 8, "Rotate A left"),
                0x00 => inst!(Rotate(RegU8::B, true, false, true, false, false), 8, "Rotate B left"),
                0x01 => inst!(Rotate(RegU8::C, true, false, true, false, false), 8, "Rotate C left"),
                0x02 => inst!(Rotate(RegU8::D, true, false, true, false, false), 8, "Rotate D left"),
                0x03 => inst!(Rotate(RegU8::E, true, false, true, false, false), 8, "Rotate E left"),
                0x04 => inst!(Rotate(RegU8::H, true, false, true, false, false), 8, "Rotate H left"),
                0x05 => inst!(Rotate(RegU8::L, true, false, true, false, false), 8, "Rotate L left"),
                0x06 => inst!(Rotate(RegU8::RamU16(RegU16::HL), true, false, true, false, false), 16, "Rotate ram[HL] left"),

                0x0F => inst!(Rotate(RegU8::A, false, false, true, false, false), 8, "Rotate A right"),
                0x08 => inst!(Rotate(RegU8::B, false, false, true, false, false), 8, "Rotate B right"),
                0x09 => inst!(Rotate(RegU8::C, false, false, true, false, false), 8, "Rotate C right"),
                0x0A => inst!(Rotate(RegU8::D, false, false, true, false, false), 8, "Rotate D right"),
                0x0B => inst!(Rotate(RegU8::E, false, false, true, false, false), 8, "Rotate E right"),
                0x0C => inst!(Rotate(RegU8::H, false, false, true, false, false), 8, "Rotate H right"),
                0x0D => inst!(Rotate(RegU8::L, false, false, true, false, false), 8, "Rotate L right"),
                0x0E => inst!(Rotate(RegU8::RamU16(RegU16::HL), false, false, true, false, false), 16, "Rotate ram[HL] right"),
                
                0x17 => inst!(Rotate(RegU8::A, true, true, true, false, false), 8, "Rotate A left through C flag"),
                0x10 => inst!(Rotate(RegU8::B, true, true, true, false, false), 8, "Rotate B left through C flag"),
                0x11 => inst!(Rotate(RegU8::C, true, true, true, false, false), 8, "Rotate C left through C flag"),
                0x12 => inst!(Rotate(RegU8::D, true, true, true, false, false), 8, "Rotate D left through C flag"),
                0x13 => inst!(Rotate(RegU8::E, true, true, true, false, false), 8, "Rotate E left through C flag"),
                0x14 => inst!(Rotate(RegU8::H, true, true, true, false, false), 8, "Rotate H left through C flag"),
                0x15 => inst!(Rotate(RegU8::L, true, true, true, false, false), 8, "Rotate L left through C flag"),
                0x16 => inst!(Rotate(RegU8::RamU16(RegU16::HL), true, true, true, false, false), 16, "Rotate ram[HL] left through C flag"),

                0x1F => inst!(Rotate(RegU8::A, false, true, true, false, false), 8, "Rotate A right through C flag"),
                0x18 => inst!(Rotate(RegU8::B, false, true, true, false, false), 8, "Rotate B right through C flag"),
                0x19 => inst!(Rotate(RegU8::C, false, true, true, false, false), 8, "Rotate C right through C flag"),
                0x1A => inst!(Rotate(RegU8::D, false, true, true, false, false), 8, "Rotate D right through C flag"),
                0x1B => inst!(Rotate(RegU8::E, false, true, true, false, false), 8, "Rotate E right through C flag"),
                0x1C => inst!(Rotate(RegU8::H, false, true, true, false, false), 8, "Rotate H right through C flag"),
                0x1D => inst!(Rotate(RegU8::L, false, true, true, false, false), 8, "Rotate L right through C flag"),
                0x1E => inst!(Rotate(RegU8::RamU16(RegU16::HL), false, true, true, false, false), 16, "Rotate ram[HL] right through C flag"),
                //Shifts
                0x27 => inst!(Rotate(RegU8::A, true, false, true, true, false), 8, "Shift A left"),
                0x20 => inst!(Rotate(RegU8::B, true, false, true, true, false), 8, "Shift B left"),
                0x21 => inst!(Rotate(RegU8::C, true, false, true, true, false), 8, "Shift C left"),
                0x22 => inst!(Rotate(RegU8::D, true, false, true, true, false), 8, "Shift D left"),
                0x23 => inst!(Rotate(RegU8::E, true, false, true, true, false), 8, "Shift E left"),
                0x24 => inst!(Rotate(RegU8::H, true, false, true, true, false), 8, "Shift H left"),
                0x25 => inst!(Rotate(RegU8::L, true, false, true, true, false), 8, "Shift L left"),
                0x26 => inst!(Rotate(RegU8::RamU16(RegU16::HL), true, false, true, true, false), 16, "Shift ram[HL] left"),

                0x2F => inst!(Rotate(RegU8::A, false, false, true, true, true), 8, "Shift A right, keep MSB"),
                0x28 => inst!(Rotate(RegU8::B, false, false, true, true, true), 8, "Shift B right, keep MSB"),
                0x29 => inst!(Rotate(RegU8::C, false, false, true, true, true), 8, "Shift C right, keep MSB"),
                0x2A => inst!(Rotate(RegU8::D, false, false, true, true, true), 8, "Shift D right, keep MSB"),
                0x2B => inst!(Rotate(RegU8::E, false, false, true, true, true), 8, "Shift E right, keep MSB"),
                0x2C => inst!(Rotate(RegU8::H, false, false, true, true, true), 8, "Shift H right, keep MSB"),
                0x2D => inst!(Rotate(RegU8::L, false, false, true, true, true), 8, "Shift L right, keep MSB"),
                0x2E => inst!(Rotate(RegU8::RamU16(RegU16::HL), false, false, true, true, true), 16, "Shift ram[HL] right, keep MSB"),

                0x3F => inst!(Rotate(RegU8::A, false, false, true, true, false), 8, "Shift A right"),
                0x38 => inst!(Rotate(RegU8::B, false, false, true, true, false), 8, "Shift B right"),
                0x39 => inst!(Rotate(RegU8::C, false, false, true, true, false), 8, "Shift C right"),
                0x3A => inst!(Rotate(RegU8::D, false, false, true, true, false), 8, "Shift D right"),
                0x3B => inst!(Rotate(RegU8::E, false, false, true, true, false), 8, "Shift E right"),
                0x3C => inst!(Rotate(RegU8::H, false, false, true, true, false), 8, "Shift H right"),
                0x3D => inst!(Rotate(RegU8::L, false, false, true, true, false), 8, "Shift L right"),
                0x3E => inst!(Rotate(RegU8::RamU16(RegU16::HL), false, false, true, true, false), 16, "Shift ram[HL] right"),
                //BIT
                0x47 => inst!(Bit(RegU8::A, 0), 8, "Set Z as the opposite of bit 0 in A"),
                0x40 => inst!(Bit(RegU8::B, 0), 8, "Set Z as the opposite of bit 0 in B"),
                0x41 => inst!(Bit(RegU8::C, 0), 8, "Set Z as the opposite of bit 0 in C"),
                0x42 => inst!(Bit(RegU8::D, 0), 8, "Set Z as the opposite of bit 0 in D"),
                0x43 => inst!(Bit(RegU8::E, 0), 8, "Set Z as the opposite of bit 0 in E"),
                0x44 => inst!(Bit(RegU8::H, 0), 8, "Set Z as the opposite of bit 0 in H"),
                0x45 => inst!(Bit(RegU8::L, 0), 8, "Set Z as the opposite of bit 0 in L"),
                0x46 => inst!(Bit(RegU8::RamU16(RegU16::HL), 0), 12, "Set Z as the opposite of bit 0 in ram[HL]"),

                0x4F => inst!(Bit(RegU8::A, 1), 8, "Set Z as the opposite of bit 1 in A"),
                0x48 => inst!(Bit(RegU8::B, 1), 8, "Set Z as the opposite of bit 1 in B"),
                0x49 => inst!(Bit(RegU8::C, 1), 8, "Set Z as the opposite of bit 1 in C"),
                0x4A => inst!(Bit(RegU8::D, 1), 8, "Set Z as the opposite of bit 1 in D"),
                0x4B => inst!(Bit(RegU8::E, 1), 8, "Set Z as the opposite of bit 1 in E"),
                0x4C => inst!(Bit(RegU8::H, 1), 8, "Set Z as the opposite of bit 1 in H"),
                0x4D => inst!(Bit(RegU8::L, 1), 8, "Set Z as the opposite of bit 1 in L"),
                0x4E => inst!(Bit(RegU8::RamU16(RegU16::HL), 1), 12, "Set Z as the opposite of bit 1 in ram[HL]"),

                0x57 => inst!(Bit(RegU8::A, 2), 8, "Set Z as the opposite of bit 2 in A"),
                0x50 => inst!(Bit(RegU8::B, 2), 8, "Set Z as the opposite of bit 2 in B"),
                0x51 => inst!(Bit(RegU8::C, 2), 8, "Set Z as the opposite of bit 2 in C"),
                0x52 => inst!(Bit(RegU8::D, 2), 8, "Set Z as the opposite of bit 2 in D"),
                0x53 => inst!(Bit(RegU8::E, 2), 8, "Set Z as the opposite of bit 2 in E"),
                0x54 => inst!(Bit(RegU8::H, 2), 8, "Set Z as the opposite of bit 2 in H"),
                0x55 => inst!(Bit(RegU8::L, 2), 8, "Set Z as the opposite of bit 2 in L"),
                0x56 => inst!(Bit(RegU8::RamU16(RegU16::HL), 2), 12, "Set Z as the opposite of bit 2 in ram[HL]"),

                0x5F => inst!(Bit(RegU8::A, 3), 8, "Set Z as the opposite of bit 3 in A"),
                0x58 => inst!(Bit(RegU8::B, 3), 8, "Set Z as the opposite of bit 3 in B"),
                0x59 => inst!(Bit(RegU8::C, 3), 8, "Set Z as the opposite of bit 3 in C"),
                0x5A => inst!(Bit(RegU8::D, 3), 8, "Set Z as the opposite of bit 3 in D"),
                0x5B => inst!(Bit(RegU8::E, 3), 8, "Set Z as the opposite of bit 3 in E"),
                0x5C => inst!(Bit(RegU8::H, 3), 8, "Set Z as the opposite of bit 3 in H"),
                0x5D => inst!(Bit(RegU8::L, 3), 8, "Set Z as the opposite of bit 3 in L"),
                0x5E => inst!(Bit(RegU8::RamU16(RegU16::HL), 3), 12, "Set Z as the opposite of bit 3 in ram[HL]"),

                0x67 => inst!(Bit(RegU8::A, 4), 8, "Set Z as the opposite of bit 4 in A"),
                0x60 => inst!(Bit(RegU8::B, 4), 8, "Set Z as the opposite of bit 4 in B"),
                0x61 => inst!(Bit(RegU8::C, 4), 8, "Set Z as the opposite of bit 4 in C"),
                0x62 => inst!(Bit(RegU8::D, 4), 8, "Set Z as the opposite of bit 4 in D"),
                0x63 => inst!(Bit(RegU8::E, 4), 8, "Set Z as the opposite of bit 4 in E"),
                0x64 => inst!(Bit(RegU8::H, 4), 8, "Set Z as the opposite of bit 4 in H"),
                0x65 => inst!(Bit(RegU8::L, 4), 8, "Set Z as the opposite of bit 4 in L"),
                0x66 => inst!(Bit(RegU8::RamU16(RegU16::HL), 4), 12, "Set Z as the opposite of bit 4 in ram[HL]"),

                0x6F => inst!(Bit(RegU8::A, 5), 8, "Set Z as the opposite of bit 5 in A"),
                0x68 => inst!(Bit(RegU8::B, 5), 8, "Set Z as the opposite of bit 5 in B"),
                0x69 => inst!(Bit(RegU8::C, 5), 8, "Set Z as the opposite of bit 5 in C"),
                0x6A => inst!(Bit(RegU8::D, 5), 8, "Set Z as the opposite of bit 5 in D"),
                0x6B => inst!(Bit(RegU8::E, 5), 8, "Set Z as the opposite of bit 5 in E"),
                0x6C => inst!(Bit(RegU8::H, 5), 8, "Set Z as the opposite of bit 5 in H"),
                0x6D => inst!(Bit(RegU8::L, 5), 8, "Set Z as the opposite of bit 5 in L"),
                0x6E => inst!(Bit(RegU8::RamU16(RegU16::HL), 5), 12, "Set Z as the opposite of bit 5 in ram[HL]"),

                0x77 => inst!(Bit(RegU8::A, 6), 8, "Set Z as the opposite of bit 6 in A"),
                0x70 => inst!(Bit(RegU8::B, 6), 8, "Set Z as the opposite of bit 6 in B"),
                0x71 => inst!(Bit(RegU8::C, 6), 8, "Set Z as the opposite of bit 6 in C"),
                0x72 => inst!(Bit(RegU8::D, 6), 8, "Set Z as the opposite of bit 6 in D"),
                0x73 => inst!(Bit(RegU8::E, 6), 8, "Set Z as the opposite of bit 6 in E"),
                0x74 => inst!(Bit(RegU8::H, 6), 8, "Set Z as the opposite of bit 6 in H"),
                0x75 => inst!(Bit(RegU8::L, 6), 8, "Set Z as the opposite of bit 6 in L"),
                0x76 => inst!(Bit(RegU8::RamU16(RegU16::HL), 6), 12, "Set Z as the opposite of bit 6 in ram[HL]"),

                0x7F => inst!(Bit(RegU8::A, 7), 8, "Set Z as the opposite of bit 7 in A"),
                0x78 => inst!(Bit(RegU8::B, 7), 8, "Set Z as the opposite of bit 7 in B"),
                0x79 => inst!(Bit(RegU8::C, 7), 8, "Set Z as the opposite of bit 7 in C"),
                0x7A => inst!(Bit(RegU8::D, 7), 8, "Set Z as the opposite of bit 7 in D"),
                0x7B => inst!(Bit(RegU8::E, 7), 8, "Set Z as the opposite of bit 7 in E"),
                0x7C => inst!(Bit(RegU8::H, 7), 8, "Set Z as the opposite of bit 7 in H"),
                0x7D => inst!(Bit(RegU8::L, 7), 8, "Set Z as the opposite of bit 7 in L"),
                0x7E => inst!(Bit(RegU8::RamU16(RegU16::HL), 7), 12, "Set Z as the opposite of bit 7 in ram[HL]"),
                //RESET
                0x87 => inst!(ResetBit(RegU8::A, 0), 8, "Reset bit 0 of A"),
                0x80 => inst!(ResetBit(RegU8::B, 0), 8, "Reset bit 0 of B"),
                0x81 => inst!(ResetBit(RegU8::C, 0), 8, "Reset bit 0 of C"),
                0x82 => inst!(ResetBit(RegU8::D, 0), 8, "Reset bit 0 of D"),
                0x83 => inst!(ResetBit(RegU8::E, 0), 8, "Reset bit 0 of E"),
                0x84 => inst!(ResetBit(RegU8::H, 0), 8, "Reset bit 0 of H"),
                0x85 => inst!(ResetBit(RegU8::L, 0), 8, "Reset bit 0 of L"),
                0x86 => inst!(ResetBit(RegU8::RamU16(RegU16::HL), 0), 16, "Reset bit 0 of ram[HL]"),

                0x8F => inst!(ResetBit(RegU8::A, 1), 8, "Reset bit 1 of A"),
                0x88 => inst!(ResetBit(RegU8::B, 1), 8, "Reset bit 1 of B"),
                0x89 => inst!(ResetBit(RegU8::C, 1), 8, "Reset bit 1 of C"),
                0x8A => inst!(ResetBit(RegU8::D, 1), 8, "Reset bit 1 of D"),
                0x8B => inst!(ResetBit(RegU8::E, 1), 8, "Reset bit 1 of E"),
                0x8C => inst!(ResetBit(RegU8::H, 1), 8, "Reset bit 1 of H"),
                0x8D => inst!(ResetBit(RegU8::L, 1), 8, "Reset bit 1 of L"),
                0x8E => inst!(ResetBit(RegU8::RamU16(RegU16::HL), 1), 16, "Reset bit 1 of ram[HL]"),

                0x97 => inst!(ResetBit(RegU8::A, 2), 8, "Reset bit 2 of A"),
                0x90 => inst!(ResetBit(RegU8::B, 2), 8, "Reset bit 2 of B"),
                0x91 => inst!(ResetBit(RegU8::C, 2), 8, "Reset bit 2 of C"),
                0x92 => inst!(ResetBit(RegU8::D, 2), 8, "Reset bit 2 of D"),
                0x93 => inst!(ResetBit(RegU8::E, 2), 8, "Reset bit 2 of E"),
                0x94 => inst!(ResetBit(RegU8::H, 2), 8, "Reset bit 2 of H"),
                0x95 => inst!(ResetBit(RegU8::L, 2), 8, "Reset bit 2 of L"),
                0x96 => inst!(ResetBit(RegU8::RamU16(RegU16::HL), 2), 16, "Reset bit 2 of ram[HL]"),

                0x9F => inst!(ResetBit(RegU8::A, 3), 8, "Reset bit 3 of A"),
                0x98 => inst!(ResetBit(RegU8::B, 3), 8, "Reset bit 3 of B"),
                0x99 => inst!(ResetBit(RegU8::C, 3), 8, "Reset bit 3 of C"),
                0x9A => inst!(ResetBit(RegU8::D, 3), 8, "Reset bit 3 of D"),
                0x9B => inst!(ResetBit(RegU8::E, 3), 8, "Reset bit 3 of E"),
                0x9C => inst!(ResetBit(RegU8::H, 3), 8, "Reset bit 3 of H"),
                0x9D => inst!(ResetBit(RegU8::L, 3), 8, "Reset bit 3 of L"),
                0x9E => inst!(ResetBit(RegU8::RamU16(RegU16::HL), 3), 16, "Reset bit 3 of ram[HL]"),

                0xA7 => inst!(ResetBit(RegU8::A, 4), 8, "Reset bit 4 of A"),
                0xA0 => inst!(ResetBit(RegU8::B, 4), 8, "Reset bit 4 of B"),
                0xA1 => inst!(ResetBit(RegU8::C, 4), 8, "Reset bit 4 of C"),
                0xA2 => inst!(ResetBit(RegU8::D, 4), 8, "Reset bit 4 of D"),
                0xA3 => inst!(ResetBit(RegU8::E, 4), 8, "Reset bit 4 of E"),
                0xA4 => inst!(ResetBit(RegU8::H, 4), 8, "Reset bit 4 of H"),
                0xA5 => inst!(ResetBit(RegU8::L, 4), 8, "Reset bit 4 of L"),
                0xA6 => inst!(ResetBit(RegU8::RamU16(RegU16::HL), 4), 16, "Reset bit 4 of ram[HL]"),

                0xAF => inst!(ResetBit(RegU8::A, 5), 8, "Reset bit 5 of A"),
                0xA8 => inst!(ResetBit(RegU8::B, 5), 8, "Reset bit 5 of B"),
                0xA9 => inst!(ResetBit(RegU8::C, 5), 8, "Reset bit 5 of C"),
                0xAA => inst!(ResetBit(RegU8::D, 5), 8, "Reset bit 5 of D"),
                0xAB => inst!(ResetBit(RegU8::E, 5), 8, "Reset bit 5 of E"),
                0xAC => inst!(ResetBit(RegU8::H, 5), 8, "Reset bit 5 of H"),
                0xAD => inst!(ResetBit(RegU8::L, 5), 8, "Reset bit 5 of L"),
                0xAE => inst!(ResetBit(RegU8::RamU16(RegU16::HL), 5), 16, "Reset bit 5 of ram[HL]"),

                0xB7 => inst!(ResetBit(RegU8::A, 6), 8, "Reset bit 6 of A"),
                0xB0 => inst!(ResetBit(RegU8::B, 6), 8, "Reset bit 6 of B"),
                0xB1 => inst!(ResetBit(RegU8::C, 6), 8, "Reset bit 6 of C"),
                0xB2 => inst!(ResetBit(RegU8::D, 6), 8, "Reset bit 6 of D"),
                0xB3 => inst!(ResetBit(RegU8::E, 6), 8, "Reset bit 6 of E"),
                0xB4 => inst!(ResetBit(RegU8::H, 6), 8, "Reset bit 6 of H"),
                0xB5 => inst!(ResetBit(RegU8::L, 6), 8, "Reset bit 6 of L"),
                0xB6 => inst!(ResetBit(RegU8::RamU16(RegU16::HL), 6), 16, "Reset bit 6 of ram[HL]"),

                0xBF => inst!(ResetBit(RegU8::A, 7), 8, "Reset bit 7 of A"),
                0xB8 => inst!(ResetBit(RegU8::B, 7), 8, "Reset bit 7 of B"),
                0xB9 => inst!(ResetBit(RegU8::C, 7), 8, "Reset bit 7 of C"),
                0xBA => inst!(ResetBit(RegU8::D, 7), 8, "Reset bit 7 of D"),
                0xBB => inst!(ResetBit(RegU8::E, 7), 8, "Reset bit 7 of E"),
                0xBC => inst!(ResetBit(RegU8::H, 7), 8, "Reset bit 7 of H"),
                0xBD => inst!(ResetBit(RegU8::L, 7), 8, "Reset bit 7 of L"),
                0xBE => inst!(ResetBit(RegU8::RamU16(RegU16::HL), 7), 16, "Reset bit 7 of ram[HL]"),
                //SET
                0xC7 => inst!(SetBit(RegU8::A, 0), 8, "Set bit 0 of A"),
                0xC0 => inst!(SetBit(RegU8::B, 0), 8, "Set bit 0 of B"),
                0xC1 => inst!(SetBit(RegU8::C, 0), 8, "Set bit 0 of C"),
                0xC2 => inst!(SetBit(RegU8::D, 0), 8, "Set bit 0 of D"),
                0xC3 => inst!(SetBit(RegU8::E, 0), 8, "Set bit 0 of E"),
                0xC4 => inst!(SetBit(RegU8::H, 0), 8, "Set bit 0 of H"),
                0xC5 => inst!(SetBit(RegU8::L, 0), 8, "Set bit 0 of L"),
                0xC6 => inst!(SetBit(RegU8::RamU16(RegU16::HL), 0), 16, "Set bit 0 of ram[HL]"),

                0xCF => inst!(SetBit(RegU8::A, 1), 8, "Set bit 1 of A"),
                0xC8 => inst!(SetBit(RegU8::B, 1), 8, "Set bit 1 of B"),
                0xC9 => inst!(SetBit(RegU8::C, 1), 8, "Set bit 1 of C"),
                0xCA => inst!(SetBit(RegU8::D, 1), 8, "Set bit 1 of D"),
                0xCB => inst!(SetBit(RegU8::E, 1), 8, "Set bit 1 of E"),
                0xCC => inst!(SetBit(RegU8::H, 1), 8, "Set bit 1 of H"),
                0xCD => inst!(SetBit(RegU8::L, 1), 8, "Set bit 1 of L"),
                0xCE => inst!(SetBit(RegU8::RamU16(RegU16::HL), 1), 16, "Set bit 1 of ram[HL]"),

                0xD7 => inst!(SetBit(RegU8::A, 2), 8, "Set bit 2 of A"),
                0xD0 => inst!(SetBit(RegU8::B, 2), 8, "Set bit 2 of B"),
                0xD1 => inst!(SetBit(RegU8::C, 2), 8, "Set bit 2 of C"),
                0xD2 => inst!(SetBit(RegU8::D, 2), 8, "Set bit 2 of D"),
                0xD3 => inst!(SetBit(RegU8::E, 2), 8, "Set bit 2 of E"),
                0xD4 => inst!(SetBit(RegU8::H, 2), 8, "Set bit 2 of H"),
                0xD5 => inst!(SetBit(RegU8::L, 2), 8, "Set bit 2 of L"),
                0xD6 => inst!(SetBit(RegU8::RamU16(RegU16::HL), 2), 16, "Set bit 2 of ram[HL]"),

                0xDF => inst!(SetBit(RegU8::A, 3), 8, "Set bit 3 of A"),
                0xD8 => inst!(SetBit(RegU8::B, 3), 8, "Set bit 3 of B"),
                0xD9 => inst!(SetBit(RegU8::C, 3), 8, "Set bit 3 of C"),
                0xDA => inst!(SetBit(RegU8::D, 3), 8, "Set bit 3 of D"),
                0xDB => inst!(SetBit(RegU8::E, 3), 8, "Set bit 3 of E"),
                0xDC => inst!(SetBit(RegU8::H, 3), 8, "Set bit 3 of H"),
                0xDD => inst!(SetBit(RegU8::L, 3), 8, "Set bit 3 of L"),
                0xDE => inst!(SetBit(RegU8::RamU16(RegU16::HL), 3), 16, "Set bit 3 of ram[HL]"),

                0xE7 => inst!(SetBit(RegU8::A, 4), 8, "Set bit 4 of A"),
                0xE0 => inst!(SetBit(RegU8::B, 4), 8, "Set bit 4 of B"),
                0xE1 => inst!(SetBit(RegU8::C, 4), 8, "Set bit 4 of C"),
                0xE2 => inst!(SetBit(RegU8::D, 4), 8, "Set bit 4 of D"),
                0xE3 => inst!(SetBit(RegU8::E, 4), 8, "Set bit 4 of E"),
                0xE4 => inst!(SetBit(RegU8::H, 4), 8, "Set bit 4 of H"),
                0xE5 => inst!(SetBit(RegU8::L, 4), 8, "Set bit 4 of L"),
                0xE6 => inst!(SetBit(RegU8::RamU16(RegU16::HL), 4), 16, "Set bit 4 of ram[HL]"),

                0xEF => inst!(SetBit(RegU8::A, 5), 8, "Set bit 5 of A"),
                0xE8 => inst!(SetBit(RegU8::B, 5), 8, "Set bit 5 of B"),
                0xE9 => inst!(SetBit(RegU8::C, 5), 8, "Set bit 5 of C"),
                0xEA => inst!(SetBit(RegU8::D, 5), 8, "Set bit 5 of D"),
                0xEB => inst!(SetBit(RegU8::E, 5), 8, "Set bit 5 of E"),
                0xEC => inst!(SetBit(RegU8::H, 5), 8, "Set bit 5 of H"),
                0xED => inst!(SetBit(RegU8::L, 5), 8, "Set bit 5 of L"),
                0xEE => inst!(SetBit(RegU8::RamU16(RegU16::HL), 5), 16, "Set bit 5 of ram[HL]"),

                0xF7 => inst!(SetBit(RegU8::A, 6), 8, "Set bit 6 of A"),
                0xF0 => inst!(SetBit(RegU8::B, 6), 8, "Set bit 6 of B"),
                0xF1 => inst!(SetBit(RegU8::C, 6), 8, "Set bit 6 of C"),
                0xF2 => inst!(SetBit(RegU8::D, 6), 8, "Set bit 6 of D"),
                0xF3 => inst!(SetBit(RegU8::E, 6), 8, "Set bit 6 of E"),
                0xF4 => inst!(SetBit(RegU8::H, 6), 8, "Set bit 6 of H"),
                0xF5 => inst!(SetBit(RegU8::L, 6), 8, "Set bit 6 of L"),
                0xF6 => inst!(SetBit(RegU8::RamU16(RegU16::HL), 6), 16, "Set bit 6 of ram[HL]"),

                0xFF => inst!(SetBit(RegU8::A, 7), 8, "Set bit 7 of A"),
                0xF8 => inst!(SetBit(RegU8::B, 7), 8, "Set bit 7 of B"),
                0xF9 => inst!(SetBit(RegU8::C, 7), 8, "Set bit 7 of C"),
                0xFA => inst!(SetBit(RegU8::D, 7), 8, "Set bit 7 of D"),
                0xFB => inst!(SetBit(RegU8::E, 7), 8, "Set bit 7 of E"),
                0xFC => inst!(SetBit(RegU8::H, 7), 8, "Set bit 7 of H"),
                0xFD => inst!(SetBit(RegU8::L, 7), 8, "Set bit 7 of L"),
                0xFE => inst!(SetBit(RegU8::RamU16(RegU16::HL), 7), 16, "Set bit 7 of ram[HL]"),
            }}
            //(Synthé keyboard) MIGHT AS WELL JUMP (JUMP)
            0xC3 => inst!(Jump(RegU16::U16, false, None, false), 16, "Jump to nn"),
            0xC2 => inst!(Jump(RegU16::U16, false, Some(Flag::Z), false), 12, "Jump to nn if Z is reset"),
            0xCA => inst!(Jump(RegU16::U16, false, Some(Flag::Z), true), 12, "Jump to nn if Z is set"),
            0xD2 => inst!(Jump(RegU16::U16, false, Some(Flag::C), false), 12, "Jump to nn if C is reset"),
            0xDA => inst!(Jump(RegU16::U16, false, Some(Flag::C), true), 12, "Jump to nn if C is set"),
            0xE9 => inst!(Jump(RegU16::HL, false, None, true), 4, "Jump to (HL)"),
            0x18 => inst!(Jump(RegU16::I8, false, None, false), 12, "Relative jump to PC+i8"),
            0x20 => inst!(Jump(RegU16::I8, false, Some(Flag::Z), false), 8, "Relative jump to PC+i8 if Z is reset"),
            0x28 => inst!(Jump(RegU16::I8, false, Some(Flag::Z), true), 8, "Relative jump to PC+i8 if Z is set"),
            0x30 => inst!(Jump(RegU16::I8, false, Some(Flag::C), false), 8, "Relative jump to PC+i8 if C is reset"),
            0x38 => inst!(Jump(RegU16::I8, false, Some(Flag::C), true), 8, "Relative jump to PC+i8 if C is set"),
            //Calls
            0xCD => inst!(Jump(RegU16::U16, true, None, false), 24, "Go to subroutine at address nn"),
            0xC4 => inst!(Jump(RegU16::U16, true, Some(Flag::Z), false), 12, "Go to subroutine at address nn if Z is reset"),
            0xCC => inst!(Jump(RegU16::U16, true, Some(Flag::Z), true), 12, "Go to subroutine at address nn if Z is set"),
            0xD4 => inst!(Jump(RegU16::U16, true, Some(Flag::C), false), 12, "Go to subroutine at address nn if C is reset"),
            0xDC => inst!(Jump(RegU16::U16, true, Some(Flag::C), true), 12, "Go to subroutine at address nn if C is set"),
            //Restart
            0xC7 => inst!(Reset(0), 16, "Push current address and reset to 0x0000"),
            0xCF => inst!(Reset(1), 16, "Push current address and reset to 0x0008"),
            0xD7 => inst!(Reset(2), 16, "Push current address and reset to 0x0010"),
            0xDF => inst!(Reset(3), 16, "Push current address and reset to 0x0018"),
            0xE7 => inst!(Reset(4), 16, "Push current address and reset to 0x0020"),
            0xEF => inst!(Reset(5), 16, "Push current address and reset to 0x0028"),
            0xF7 => inst!(Reset(6), 16, "Push current address and reset to 0x0030"),
            0xFF => inst!(Reset(7), 16, "Push current address and reset to 0x0038"),
            //Returns
            0xC9 => inst!(Ret(None, false, false), 16, "Return from subroutine"),
            0xC0 => inst!(Ret(Some(Flag::Z), false, false), 8, "Return from subroutine if Z is reset"),
            0xC8 => inst!(Ret(Some(Flag::Z), true, false), 8, "Return from subroutine if Z is set"),
            0xD0 => inst!(Ret(Some(Flag::C), false, false), 8, "Return from subroutine if C is reset"),
            0xD8 => inst!(Ret(Some(Flag::C), true, false), 8, "Return from subroutine if C is set"),
            0xD9 => inst!(Ret(None, false, true), 16, "Return from subroutine and enable interrupts"),
            //Undefined
            0xD3 => inst!(Nop, 4, "Undefined NOP"),
            0xE3 => inst!(Nop, 4, "Undefined NOP"),
            0xE4 => inst!(Nop, 4, "Undefined NOP"),
            0xF4 => inst!(Nop, 4, "Undefined NOP"),
            0xDB => inst!(Nop, 4, "Undefined NOP"),
            0xEB => inst!(Nop, 4, "Undefined NOP"),
            0xEC => inst!(Nop, 4, "Undefined NOP"),
            0xFC => inst!(Nop, 4, "Undefined NOP"),
            0xDD => inst!(Nop, 4, "Undefined NOP"),
            0xED => inst!(Nop, 4, "Undefined NOP"),
            0xFD => inst!(Nop, 4, "Undefined NOP"),
        }
    }
}

use crate::hardware::{Flag, RegU8, RegU16, Cpu};
use crate::memory::Memory;
use std::fmt;

mod instruct_fn;

pub struct Instruct {
    pub opcode: u8,
    pub inst: InstructType,
    pub desc: String,
    pub ticks: u8,
}

pub enum InstructType {
    Load(RegU8, RegU8), // Source, destination
    LoadPlus(RegU8, RegU8, bool), // Source, destination, increase (true) / decrease (false)
    LoadU16(RegU16, RegU16), // Source, destination
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
    pub fn exec(&self, cpu: &mut Cpu, mem: &mut Memory) {
        match self {
            Load(src, dest) => instruct_fn::load(cpu, mem, src, dest),
            LoadPlus(src, dest, inc) => instruct_fn::load_plus(cpu, mem, src, dest, *inc),
            LoadU16(src, dest) => instruct_fn::load_u16(cpu, mem, src, dest),
            Push(src) => instruct_fn::push(cpu, mem, src),
            Pop(dest) => instruct_fn::pop(cpu, mem, dest),
            Add(src, carry) => instruct_fn::add(cpu, mem, src, *carry),
            Sub(src, carry) => instruct_fn::sub(cpu, mem, src, *carry),
            And(src) => instruct_fn::and(cpu, mem, src),
            Or(src) => instruct_fn::or(cpu, mem, src),
            Xor(src) => instruct_fn::xor(cpu, mem, src),
            Cmp(src) => { instruct_fn::cmp(cpu, mem, src, false); },
            Inc(reg) => instruct_fn::inc(cpu, mem, reg),
            Dec(reg) => instruct_fn::dec(cpu, mem, reg),
            AddU16(src) => instruct_fn::add_u16(cpu, mem, src),
            AddU16I8(dest, offset) => instruct_fn::add_u16_i8(cpu, mem, dest, offset),
            IncU16(reg) => instruct_fn::inc_u16(cpu, mem, reg),
            DecU16(reg) => instruct_fn::dec_u16(cpu, mem, reg),
            Daa => instruct_fn::daa(cpu, mem),
            Cpl => instruct_fn::cpl(cpu, mem),
            SetCarry(flip) => instruct_fn::set_carry(cpu, mem, *flip),
            Nop => {},
            ChangeMie(enable) => instruct_fn::change_mie(cpu, mem, *enable),
            Rotate(reg, left, through_carry, update_z, shift, keep_msb) => 
                instruct_fn::rotate(cpu, mem, reg, *left, *through_carry, *update_z, *shift, *keep_msb),
            Swap(reg) => instruct_fn::swap(cpu, mem, reg),
            Bit(reg, bit_pos) => instruct_fn::bit(cpu, mem, reg, *bit_pos),
            SetBit(reg, bit_pos) => instruct_fn::set_bit(cpu, mem, reg, *bit_pos),
            ResetBit(reg, bit_pos) => instruct_fn::reset_bit(cpu, mem, reg, *bit_pos),
            Jump(addr, is_call, flag, is_set) => instruct_fn::jump(cpu, mem, addr, *is_call, flag.as_ref(), *is_set),
            Reset(nth_byte) => instruct_fn::reset(cpu, mem, *nth_byte),
            Ret(flag, is_set, i_enable) => instruct_fn::ret(cpu, mem, flag.as_ref(), *is_set, *i_enable),
        }
    }
}

impl fmt::Display for InstructType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Load(src, dest) => write!(f, "LD {} {}", dest, src),
            LoadPlus(src, dest, inc) => write!(f, "LD{} {} {}", if *inc { "I" } else { "D" }, dest, src),
            LoadU16(src, dest) => write!(f, "LD {} {}", dest, src),
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

        macro_rules! all_regs {
            // $hl_ticks is the specific duration for (HL), because it's always different
            // The variable RegU8 is always the first element of the instruction (the enum has been built that way for this purpose)
            // All other elements have to be put after the instruction description (this is the limitation of using a repetition)
            // Also, the repetition voodoo magic is here to allow 0 or more arguments
            ($opcode:ident, $inst:path, $ticks:expr, $hl_ticks:expr, $desc:expr $(, $inst_args:expr)*) => {
                match $opcode & 0x07 {
                    0x07 => inst!($inst(RegU8::A, $($inst_args),*), $ticks, format!($desc, RegU8::A)),
                    0x00 => inst!($inst(RegU8::B, $($inst_args),*), $ticks, format!($desc, RegU8::B)),
                    0x01 => inst!($inst(RegU8::C, $($inst_args),*), $ticks, format!($desc, RegU8::C)),
                    0x02 => inst!($inst(RegU8::D, $($inst_args),*), $ticks, format!($desc, RegU8::D)),
                    0x03 => inst!($inst(RegU8::E, $($inst_args),*), $ticks, format!($desc, RegU8::E)),
                    0x04 => inst!($inst(RegU8::H, $($inst_args),*), $ticks, format!($desc, RegU8::H)),
                    0x05 => inst!($inst(RegU8::L, $($inst_args),*), $ticks, format!($desc, RegU8::L)),
                    0x06 => inst!($inst(RegU8::RamU16(RegU16::HL), $($inst_args),*), $hl_ticks, format!($desc, RegU8::RamU16(RegU16::HL))),
                    _ => unreachable!(),
                }
            };
        }

        match opcode {
            //Load A
            0x3E => inst!(Load(RegU8::U8, RegU8::A), 8, "Load n in A"),
            0x78..=0x7F => all_regs!(opcode, Load, 4, 8, "Load {} in A", RegU8::A),
            0x0A => inst!(Load(RegU8::RamU16(RegU16::BC), RegU8::A), 8, "Load ram[BC] in A"),
            0x1A => inst!(Load(RegU8::RamU16(RegU16::DE), RegU8::A), 8, "Load ram[DE] in A"),
            0xFA => inst!(Load(RegU8::RamU16(RegU16::U16), RegU8::A), 16, "Load ram[nn] in A"),
            //Load B
            0x06 => inst!(Load(RegU8::U8, RegU8::B), 8, "Load n in B"),
            0x40..=0x47 => all_regs!(opcode, Load, 4, 8, "Load {} in B", RegU8::B),
            //Load C
            0x0E => inst!(Load(RegU8::U8, RegU8::C), 8, "Load n in C"),
            0x48..=0x4F => all_regs!(opcode, Load, 4, 8, "Load {} in C", RegU8::C),
            //Load D
            0x16 => inst!(Load(RegU8::U8, RegU8::D), 8, "Load n in D"),
            0x50..=0x57 => all_regs!(opcode, Load, 4, 8, "Load {} in D", RegU8::D),
            //Load E
            0x1E => inst!(Load(RegU8::U8, RegU8::E), 8, "Load n in E"),
            0x58..=0x5F => all_regs!(opcode, Load, 4, 8, "Load {} in E", RegU8::E),
            //Load H
            0x26 => inst!(Load(RegU8::U8, RegU8::H), 8, "Load n in H"),
            0x60..=0x67 => all_regs!(opcode, Load, 4, 8, "Load {} in H", RegU8::H),
            //Load L
            0x2E => inst!(Load(RegU8::U8, RegU8::L), 8, "Load n in L"),
            0x68..=0x6F => all_regs!(opcode, Load, 4, 8, "Load {} in L", RegU8::L),
            //Load (HL)
            0x36 => inst!(Load(RegU8::U8, RegU8::RamU16(RegU16::HL)), 12, "Store n in ram[HL]"),
            0x77 => inst!(Load(RegU8::A, RegU8::RamU16(RegU16::HL)), 8, "Store A in ram[HL]"),
            0x70 => inst!(Load(RegU8::B, RegU8::RamU16(RegU16::HL)), 8, "Store B in ram[HL]"),
            0x71 => inst!(Load(RegU8::C, RegU8::RamU16(RegU16::HL)), 8, "Store C in ram[HL]"),
            0x72 => inst!(Load(RegU8::D, RegU8::RamU16(RegU16::HL)), 8, "Store D in ram[HL]"),
            0x73 => inst!(Load(RegU8::E, RegU8::RamU16(RegU16::HL)), 8, "Store E in ram[HL]"),
            0x74 => inst!(Load(RegU8::H, RegU8::RamU16(RegU16::HL)), 8, "Store H in ram[HL]"),
            0x75 => inst!(Load(RegU8::L, RegU8::RamU16(RegU16::HL)), 8, "Store L in ram[HL]"),
            //Store A in ram
            0x02 => inst!(Load(RegU8::A, RegU8::RamU16(RegU16::BC)), 8, "Store A in ram[BC]"),
            0x12 => inst!(Load(RegU8::A, RegU8::RamU16(RegU16::DE)), 8, "Store A in ram[DE]"),
            0xEA => inst!(Load(RegU8::A, RegU8::RamU16(RegU16::U16)), 16, "Store A in ram[nn]"),
            //Load A and HRam + C
            0xF2 => inst!(Load(RegU8::RamU8(Box::new(RegU8::C)), RegU8::A), 8, "Load ram[0xFF00 + C] in A"),
            0xE2 => inst!(Load(RegU8::A, RegU8::RamU8(Box::new(RegU8::C))), 8, "Store A in ram[0xFF00 + C]"),
            //Load A and decrease/increase HL
            0x3A => inst!(LoadPlus(RegU8::RamU16(RegU16::HL), RegU8::A, false), 8, "Load ram[HL] in A, HL--"),
            0x32 => inst!(LoadPlus(RegU8::A, RegU8::RamU16(RegU16::HL), false), 8, "Store A in ram[HL], HL--"),
            0x2A => inst!(LoadPlus(RegU8::RamU16(RegU16::HL), RegU8::A, true), 8, "Load ram[HL] in A, HL++"),
            0x22 => inst!(LoadPlus(RegU8::A, RegU8::RamU16(RegU16::HL), true), 8, "Store A in ram[HL], HL++"),
            //Load A and HRam + n
            0xF0 => inst!(Load(RegU8::RamU8(Box::new(RegU8::U8)), RegU8::A), 12, "Load ram[0xFF00 + n] in A"),
            0xE0 => inst!(Load(RegU8::A, RegU8::RamU8(Box::new(RegU8::U8))), 12, "Store A in ram[0xFF00 + n]"),
            //16 bits direct loads
            0x01 => inst!(LoadU16(RegU16::U16, RegU16::BC), 12, "Load nn in BC"),
            0x11 => inst!(LoadU16(RegU16::U16, RegU16::DE), 12, "Load nn in DE"),
            0x21 => inst!(LoadU16(RegU16::U16, RegU16::HL), 12, "Load nn in HL"),
            0x31 => inst!(LoadU16(RegU16::U16, RegU16::SP), 12, "Load nn in SP"),
            //SP related loads
            0xF9 => inst!(LoadU16(RegU16::HL, RegU16::SP), 8, "Load HL in SP"),
            0xF8 => inst!(AddU16I8(RegU16::HL, RegU16::I8), 12, "Load SP+i8 in HL"),
            0x08 => inst!(LoadU16(RegU16::SP, RegU16::RamU16(Box::new(RegU16::U16))), 20, "Store SP in ram[nn]"),
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
            0x80..=0x87 => all_regs!(opcode, Add, 4, 8, "Add {} to A", false),
            //Add n + carry flag to A
            0xCE => inst!(Add(RegU8::U8, true), 8, "Add n + Cflag to A"),
            0x88..=0x8F => all_regs!(opcode, Add, 4, 8, "Add {} to A", true),
            //Sub n from A
            0xD6 => inst!(Sub(RegU8::U8, false), 8, "Sub n from A"),
            0x90..=0x97 => all_regs!(opcode, Sub, 4, 8, "Sub {} from A", false),
            //Sub n + carry flag from A
            0xDE => inst!(Sub(RegU8::U8, true), 8, "Sub n + Cflag from A"),
            0x98..=0x9F => all_regs!(opcode, Sub, 4, 8, "Sub {} + Cflag from A", true),
            //AND
            0xE6 => inst!(And(RegU8::U8), 8, "Compute n & A"),
            0xA0..=0xA7 => all_regs!(opcode, And, 4, 8, "Compute {} & A"),
            //OR
            0xF6 => inst!(Or(RegU8::U8), 8, "Compute n | A"),
            0xB0..=0xB7 => all_regs!(opcode, Or, 4, 8, "Compute {} | A"),
            //XOR (^ = XOR)
            0xEE => inst!(Xor(RegU8::U8), 8, "Compute n ^ A"),
            0xA8..=0xAF => all_regs!(opcode, Xor, 4, 8, "Compute {} ^ A"),
            //CMP (Z if A=n, C if A<n)
            0xFE => inst!(Cmp(RegU8::U8), 8, "Compare n and A"),
            0xB8..=0xBF => all_regs!(opcode, Cmp, 4, 8, "Compare {} and A"),
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
                0x30..=0x37 => all_regs!(followup_byte, Swap, 8, 16, "Swap upper and lower nibble of {}"),
                //Rotates
                0x00..=0x07 => all_regs!(followup_byte, Rotate, 8, 16, "Rotate {} left", true, false, true, false, false),
                0x08..=0x0F => all_regs!(followup_byte, Rotate, 8, 16, "Rotate {} right", false, false, true, false, false),
                0x10..=0x17 => all_regs!(followup_byte, Rotate, 8, 16, "Rotate {} left through C flag", true, true, true, false, false),
                0x18..=0x1F => all_regs!(followup_byte, Rotate, 8, 16, "Rotate {} right through C flag", false, true, true, false, false),
                //Shifts
                0x20..=0x27 => all_regs!(followup_byte, Rotate, 8, 16, "Shift {} left", true, false, true, true, false),
                0x28..=0x2F => all_regs!(followup_byte, Rotate, 8, 16, "Shift {} right, keep MSB", false, false, true, true, true),
                0x38..=0x3F => all_regs!(followup_byte, Rotate, 8, 16, "Shift {} right", false, false, true, true, false),
                //BIT
                0x40..=0x47 => all_regs!(followup_byte, Bit, 8, 12, "Set Z as the opposite of {} bit 0", 0),
                0x48..=0x4F => all_regs!(followup_byte, Bit, 8, 12, "Set Z as the opposite of {} bit 1", 1),
                0x50..=0x57 => all_regs!(followup_byte, Bit, 8, 12, "Set Z as the opposite of {} bit 2", 2),
                0x58..=0x5F => all_regs!(followup_byte, Bit, 8, 12, "Set Z as the opposite of {} bit 3", 3),
                0x60..=0x67 => all_regs!(followup_byte, Bit, 8, 12, "Set Z as the opposite of {} bit 4", 4),
                0x68..=0x6F => all_regs!(followup_byte, Bit, 8, 12, "Set Z as the opposite of {} bit 5", 5),
                0x70..=0x77 => all_regs!(followup_byte, Bit, 8, 12, "Set Z as the opposite of {} bit 6", 6),
                0x78..=0x7F => all_regs!(followup_byte, Bit, 8, 12, "Set Z as the opposite of {} bit 7", 7),
                //RESET
                0x80..=0x87 => all_regs!(followup_byte, ResetBit, 8, 16, "Reset bit 0 of {}", 0),
                0x88..=0x8F => all_regs!(followup_byte, ResetBit, 8, 16, "Reset bit 1 of {}", 1),
                0x90..=0x97 => all_regs!(followup_byte, ResetBit, 8, 16, "Reset bit 2 of {}", 2),
                0x98..=0x9F => all_regs!(followup_byte, ResetBit, 8, 16, "Reset bit 3 of {}", 3),
                0xA0..=0xA7 => all_regs!(followup_byte, ResetBit, 8, 16, "Reset bit 4 of {}", 4),
                0xA8..=0xAF => all_regs!(followup_byte, ResetBit, 8, 16, "Reset bit 5 of {}", 5),
                0xB0..=0xB7 => all_regs!(followup_byte, ResetBit, 8, 16, "Reset bit 6 of {}", 6),
                0xB8..=0xBF => all_regs!(followup_byte, ResetBit, 8, 16, "Reset bit 7 of {}", 7),
                //SET
                0xC0..=0xC7 => all_regs!(followup_byte, SetBit, 8, 16, "Set bit 0 of {}", 0),
                0xC8..=0xCF => all_regs!(followup_byte, SetBit, 8, 16, "Set bit 1 of {}", 1),
                0xD0..=0xD7 => all_regs!(followup_byte, SetBit, 8, 16, "Set bit 2 of {}", 2),
                0xD8..=0xDF => all_regs!(followup_byte, SetBit, 8, 16, "Set bit 3 of {}", 3),
                0xE0..=0xE7 => all_regs!(followup_byte, SetBit, 8, 16, "Set bit 4 of {}", 4),
                0xE8..=0xEF => all_regs!(followup_byte, SetBit, 8, 16, "Set bit 5 of {}", 5),
                0xF0..=0xF7 => all_regs!(followup_byte, SetBit, 8, 16, "Set bit 6 of {}", 6),
                0xF8..=0xFF => all_regs!(followup_byte, SetBit, 8, 16, "Set bit 7 of {}", 7),
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
            0xD3 | 0xE3 | 0xE4 | 0xF4 | 0xDB | 0xEB | 0xEC | 0xFC | 0xDD | 0xED | 0xFD => inst!(Nop, 4, "Undefined NOP"),
        }
    }
}

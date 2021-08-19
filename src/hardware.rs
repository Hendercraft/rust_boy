
use serde::{Deserialize, Serialize};
use std::fmt;

const BG: u8 = 1;
const SPRITE: u8 = 3;
const WINDOW: u8 = 2;

pub enum RegU8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    U8,
    RamU8(Box<RegU8>),
    RamU16(RegU16),
}

#[derive(PartialEq)]
pub enum RegU16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    U16,
    I8,
    RamU16(Box<RegU16>),
}

impl fmt::Display for RegU8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegU8::A => write!(f, "A"),
            RegU8::B => write!(f, "B"),
            RegU8::C => write!(f, "C"),
            RegU8::D => write!(f, "D"),
            RegU8::E => write!(f, "E"),
            RegU8::H => write!(f, "H"),
            RegU8::L => write!(f, "L"),
            RegU8::U8 => write!(f, "n"),
            RegU8::RamU8(reg_u8) => write!(f, "({})", reg_u8),
            RegU8::RamU16(reg_u16) => write!(f, "({})", reg_u16),
        }
    }
}

impl fmt::Display for RegU16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegU16::AF => write!(f, "AF"),
            RegU16::BC => write!(f, "BC"),
            RegU16::DE => write!(f, "DE"),
            RegU16::HL => write!(f, "HL"),
            RegU16::SP => write!(f, "SP"),
            RegU16::U16 => write!(f, "nn"),
            RegU16::I8 => write!(f, "i8"),
            RegU16::RamU16(reg_u16) => write!(f, "({})", reg_u16),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cpu {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub mie: bool,
    pub pending_mie: Option<bool>,
    pub pending_ticks: u8,
}

impl Cpu {
    fn get_u16(high: u8, low: u8) -> u16 {
        ((high as u16) << 8) | low as u16
    }
    
    fn set_u16(high: &mut u8, low: &mut u8, nn: u16) {
        *high = (nn >> 8) as u8;
        *low = nn as u8;
    }
    
    pub fn get_reg_u8(&mut self, ram: &[u8; 0x10000], reg: &RegU8) -> u8 {
        match reg {
            RegU8::A => self.a,
            RegU8::B => self.b,
            RegU8::C => self.c,
            RegU8::D => self.d,
            RegU8::E => self.e,
            RegU8::H => self.h,
            RegU8::L => self.l,
            RegU8::U8 => self.get_op(ram),
            RegU8::RamU8(reg_u8) => ram[0xff00 + self.get_reg_u8(ram, reg_u8) as usize],
            RegU8::RamU16(reg_u16) => ram[self.get_reg_u16(ram, reg_u16) as usize],
        }
    }

    pub fn get_reg_u16(&mut self, ram: &[u8; 0x10000], reg: &RegU16) -> u16 {
        match reg {
            RegU16::AF => Cpu::get_u16(self.a, self.f),
            RegU16::BC => Cpu::get_u16(self.b, self.c),
            RegU16::DE => Cpu::get_u16(self.d, self.e),
            RegU16::HL => Cpu::get_u16(self.h, self.l),
            RegU16::SP => self.sp,
            RegU16::U16 => {
                let low = self.get_op(ram) as u16;
                ((self.get_op(ram) as u16) << 8) + low
            },
            // A cast from u8 to i8 keeps the bits the way they are
            // A cast from i8 to u16 just works (the msb is duplicated to fill the whole upper byte)
            // Note that directly casting from u8 to u16 wouldn't work, as it would always have a null upper byte
            RegU16::I8 => (self.get_op(ram) as i8) as u16,
            RegU16::RamU16(_) => panic!("This enum variant should only be used with set_reg_u16()"),
        }
    }

    pub fn set_reg_u8(&mut self, ram: &mut [u8; 0x10000], reg: &RegU8, n: u8) {
        match reg {
            RegU8::A => self.a = n,
            RegU8::B => self.b = n,
            RegU8::C => self.c = n,
            RegU8::D => self.d = n,
            RegU8::E => self.e = n,
            RegU8::H => self.h = n,
            RegU8::L => self.l = n,
            RegU8::U8 => panic!("This enum variant should only be used with get_reg_u8()"),
            RegU8::RamU8(reg_u8) => {
                let addr_low = self.get_reg_u8(ram, reg_u8);
                self.write(ram, n, 0xff00 + addr_low as u16);
            },
            RegU8::RamU16(reg_u16) => {
                let addr = self.get_reg_u16(ram, reg_u16);
                self.write(ram, n, addr);
            },
        }
    }

    pub fn set_reg_u16(&mut self, ram: &mut [u8; 0x10000], reg: &RegU16, nn: u16) {
        match reg {
            RegU16::AF => Cpu::set_u16(&mut self.a, &mut self.f, nn),
            RegU16::BC => Cpu::set_u16(&mut self.b, &mut self.c, nn),
            RegU16::DE => Cpu::set_u16(&mut self.d, &mut self.e, nn),
            RegU16::HL => Cpu::set_u16(&mut self.h, &mut self.l, nn),
            RegU16::SP => self.sp = nn,
            RegU16::U16 => panic!("This enum variant should only be used with get_reg_u16()"),
            RegU16::I8 => panic!("This enum variant should only be used with get_reg_u16()"),
            RegU16::RamU16(reg_u16) => {
                let addr = self.get_reg_u16(ram, reg_u16);
                self.write(ram, nn as u8, addr); // Low
                self.write(ram, (nn >> 8) as u8, addr.wrapping_add(1)); // High
            }
        }
    }

    pub fn update_interrupt_status(&mut self) {
        if let Some(new_status) = self.pending_mie {
            self.mie = new_status;
            self.pending_mie = None;
        }
    }
    
    //Flags
    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => self.f & 0b1000_0000 > 0,
            Flag::N => self.f & 0b0100_0000 > 0,
            Flag::H => self.f & 0b0010_0000 > 0,
            Flag::C => self.f & 0b0001_0000 > 0,
        }
    }

    pub fn flag(&mut self, flag: Flag, state: bool) {
        let bit = match flag {
            Flag::Z => 0b10000000,
            Flag::N => 0b01000000,
            Flag::H => 0b00100000,
            Flag::C => 0b00010000,
        };

        match state {
            true => self.f |= bit,   // SET
            false => self.f &= !bit, // RESET
        }
    }

    fn get_op(&mut self, ram: &[u8; 0x10000]) -> u8 {
        let n = ram[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        
        n
    }

    pub fn write(&mut self, ram: &mut [u8; 0x10000], n: u8, addr: u16) {
        match addr {
            0x0000..=0x7FFF => {}
            0xff04 => {
                ram[0xff04] = 0;
            }
            _ => {
                ram[addr as usize] = n;
            }
        }
    }

    // Memory manipulation

    pub fn write_u16_to_stack(&mut self, n: u16, ram: &mut [u8; 0x10000]) {
        self.sp = self.sp.wrapping_sub(2);
        ram[self.sp as usize] = n as u8;
        ram[(self.sp + 1) as usize] = (n >> 8) as u8;
    }

    pub fn read_u16_from_stack(&mut self, ram: &[u8; 0x10000]) -> u16 {
        let h: u8 = ram[(self.sp + 1) as usize];
        let l: u8 = ram[self.sp as usize];
        let value: u16 = ((h as u16) << 8) | l as u16;
        
        self.sp = self.sp.wrapping_add(2);
        value
    }

    //ticks voodoo magic

    pub fn add_ticks(&mut self, ticks: u8) {
        self.pending_ticks = self.pending_ticks.wrapping_add(ticks);
    }

    pub fn get_ticks(&self) -> u8 {
        self.pending_ticks
    }

    pub fn clear_ticks(&mut self) {
        self.pending_ticks = 0;
    }
}

#[derive(Clone, Copy)]
pub enum Flag {
    Z,
    N,
    H,
    C,
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Flag::Z => write!(f, "Z"),
            Flag::N => write!(f, "N"),
            Flag::H => write!(f, "H"),
            Flag::C => write!(f, "C"),
        }
    }
}

pub struct Gpu {
    pub screen: [[u8; 144]; 160],
    pub bg_matrix: [[u8; 256]; 256],
    pub window_matrix: [[u8; 256]; 256],
    pub sprite_matrix: [[u8; 256]; 256],
    pub line: u8,
}

impl Gpu {
    fn get_tile_method(&self, ram: &[u8; 0x10000]) -> u16 {
        if ram[0xff40] & 0b00010000 > 0 {
            //println!("0x8000");
            return 0x8000;
        } else {
            //println!("0x8800");
            return 0x8800;
        }
    }

    fn get_bg_map_index(&self, ram: &[u8; 0x10000]) -> u16 {
        if ram[0xff40] & 0b00001000 > 0 {
            //println!("0x9c00");
            return 0x9c00;
        } else {
            //println!("0x9800");
            return 0x9800;
        }
    }

    fn get_window_map_index(&self, ram: &[u8; 0x10000]) -> u16 {
        if ram[0xff40] & 0b01000000 > 0 {
            //println!("0x9c00");
            return 0x9c00;
        } else {
            //println!("0x9800");
            return 0x9800;
        }
    }

    fn get_tile(&self, method: u16, mut index: u8, _ram: &[u8; 0x10000]) -> u16 {
        if method == 0x8000 {
            return 0x8000 + (index as u16) * 16;
        } else {
            if index > 127 {
                index -= 128;
                return 0x8800 + (index as u16) * 16;
            } else {
                return 0x9000 + (index as u16) * 16;
            }
        }
    }

    fn display_tile(&mut self, dest: u8, x: u8, y: u8, location: u16, ram: &[u8; 0x10000]) {
        let &mut mat;
        if dest == WINDOW {
            mat = &mut self.window_matrix;
        } else if dest == SPRITE {
            mat = &mut self.sprite_matrix;
        } else {
            mat = &mut self.bg_matrix;
        }
        for i in 0..8 {
            let mut value: u8 = 0b10000000;
            for j in 0..7 {
                mat[(x.wrapping_add(j)) as usize][(y.wrapping_add(i)) as usize] =
                    (ram[(location + 2 * (i as u16)) as usize] & value) >> 7 - j
                        | (ram[(location + 2 * (i as u16) + 1) as usize] & value) >> 6 - j;
                value = value >> 1;
            }
            mat[(x.wrapping_add(7)) as usize][(y.wrapping_add(i)) as usize] =
                (ram[(location + 2 * (i as u16)) as usize] & value)
                    | (ram[(location + 2 * (i as u16) + 1) as usize] & value) << 1;
        }
    }

    pub fn build_bg(&mut self, ram: &[u8; 0x10000]) {
        let mut n: u16 = 0;

        let index = self.get_bg_map_index(&ram);
        let method: u16 = self.get_tile_method(&ram);

        for i in 0..32 {
            for j in 0..32 {
                self.display_tile(
                    BG,
                    j * 8 as u8,
                    i * 8 as u8,
                    self.get_tile(method, ram[(index + n) as usize], &ram),
                    &ram,
                );
                n += 1;
            }
        }
    }

    pub fn build_window(&mut self, ram: &[u8; 0x10000]) {
        let mut n: u16 = 0;

        let index = self.get_window_map_index(&ram);
        let method: u16 = self.get_tile_method(&ram);

        for i in 0..32 {
            for j in 0..32 {
                //println!("Map adress: 0x{:x}",index+n);
                self.display_tile(
                    WINDOW,
                    (j * 8) as u8,
                    (i * 8) as u8,
                    self.get_tile(method, ram[(index + n) as usize], &ram),
                    &ram,
                );
                n += 1;
            }
        }
    }

    pub fn build_sprite(&mut self, ram: &[u8; 0x10000]) {
        for i in 0..255 {
            for j in 0..255 {
                self.sprite_matrix[i][j] = 0;
            }
        }

        let method: u16 = 0x8000;
        let mut sprite_index: u16;
        let mut x: u8;
        let mut y: u8;
        let mut index: u8;

        for i in 0..40 {
            sprite_index = (0xFE00 + (4 * i)) as u16;
            x = ram[(sprite_index + 1) as usize].wrapping_sub(8);
            y = ram[(sprite_index) as usize].wrapping_sub(16);
            index = ram[(sprite_index + 2) as usize];
            self.display_tile(SPRITE, x, y, self.get_tile(method, index, &ram), &ram);
        }
    }

    pub fn push_line(&mut self, ram: &[u8; 0x10000]) {
        let scroll_x: u8 = ram[0xff43];
        let scroll_y: u8 = ram[0xff42];
        let win_x: u8 = ram[0xff4b].wrapping_sub(7);
        let win_y: u8 = ram[0xff4a];

        if ram[0xff40] & 0b10000000 > 0 {
            for i in 0..160 {
                if win_x <= i && win_y <= self.line && false {
                    // && false{
                    self.screen[i as usize][self.line as usize] =
                        self.window_matrix[(i - win_x) as usize][(self.line - win_y) as usize];
                } else {
                    self.screen[i as usize][self.line as usize] = self.bg_matrix
                        [(scroll_x.wrapping_add(i)) as usize]
                        [(scroll_y.wrapping_add(self.line)) as usize];
                }
                if self.sprite_matrix[i as usize][self.line as usize] != 0 {
                    self.screen[i as usize][self.line as usize] =
                        self.sprite_matrix[i as usize][self.line as usize];
                }
            }
        } else {
            for i in 0..160 {
                self.screen[i as usize][self.line as usize] = 0;
            }
        }
        self.line += 1;
        if self.line == 144 {
            self.line = 0;
        }
    }
}

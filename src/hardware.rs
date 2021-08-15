use serde::{Serialize, Deserialize};

const BG: u8 = 1;
const SPRITE: u8 = 3;
const WINDOW: u8 = 2;

pub enum Op {
    No(fn(&mut Cpu)),
    U8(fn(&mut Cpu, u8)),
    U16(fn(&mut Cpu, u8, u8)), //High, low
    Ram(fn(&mut Cpu, &mut [u8; 0x10000])),
    RamU8(fn(&mut Cpu, u8, &mut [u8; 0x10000])),
    RamU16(fn(&mut Cpu, u8, u8, &mut [u8; 0x10000])), //High, low, ram
}

pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
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
    pub pending_ticks: u8,
}

impl Cpu {
    //getter u8
    pub fn get_a(&self) -> u8 {
        self.a
    }
    pub fn get_f(&self) -> u8 {
        self.f
    }
    pub fn get_b(&self) -> u8 {
        self.b
    }
    pub fn get_c(&self) -> u8 {
        self.c
    }
    pub fn get_d(&self) -> u8 {
        self.d
    }
    pub fn get_e(&self) -> u8 {
        self.e
    }
    pub fn get_h(&self) -> u8 {
        self.h
    }
    pub fn get_l(&self) -> u8 {
        self.l
    }
    //getter u16
    pub fn get_af(&self) -> u16 {
        Cpu::get_u16(self.a, self.f)
    }
    pub fn get_bc(&self) -> u16 {
        Cpu::get_u16(self.b, self.c)
    }
    pub fn get_de(&self) -> u16 {
        Cpu::get_u16(self.d, self.e)
    }
    pub fn get_hl(&self) -> u16 {
        Cpu::get_u16(self.h, self.l)
    }
    pub fn get_sp(&self) -> u16 {
        self.sp
    }
    pub fn get_pc(&self) -> u16 {
        self.pc
    }
    pub fn get_mie(&self) -> bool {
        self.mie
    }

    pub(crate) fn get_u16(h: u8, l: u8) -> u16 {
        ((h as u16) << 8) | l as u16
    }

    //setter u8
    pub fn set_a(&mut self, n: u8) {
        self.a = n;
    }
    pub fn set_f(&mut self, n: u8) {
        self.f = n;
    }
    pub fn set_b(&mut self, n: u8) {
        self.b = n;
    }
    pub fn set_c(&mut self, n: u8) {
        self.c = n;
    }
    pub fn set_d(&mut self, n: u8) {
        self.d = n;
    }
    pub fn set_e(&mut self, n: u8) {
        self.e = n;
    }
    pub fn set_h(&mut self, n: u8) {
        self.h = n;
    }
    pub fn set_l(&mut self, n: u8) {
        self.l = n;
    }

    //setter u16
    pub fn set_af(&mut self, n: u16) {
        Cpu::set_u16(&mut self.a, &mut self.f, n);
    }
    pub fn set_bc(&mut self, n: u16) {
        Cpu::set_u16(&mut self.b, &mut self.c, n);
    }
    pub fn set_de(&mut self, n: u16) {
        Cpu::set_u16(&mut self.d, &mut self.e, n);
    }
    pub fn set_hl(&mut self, n: u16) {
        Cpu::set_u16(&mut self.h, &mut self.l, n);
    }
    pub fn set_sp(&mut self, n: u16) {
        self.sp = n;
    }
    pub fn set_pc(&mut self, n: u16) {
        self.pc = n;
    }

    fn set_u16(h: &mut u8, l: &mut u8, n: u16) {
        *h = (n >> 8) as u8;
        *l = n as u8;
    }
    pub fn set_mie(&mut self, b: bool) {
        self.mie = b;
    }

    pub fn get_flags(&self) -> Flags {
        let temp = self.f >> 4;
        let output: Flags = Flags {
            z: temp & 0b1000 > 0, //get upper
            n: temp & 0b0100 > 0,
            h: temp & 0b0010 > 0,
            c: temp & 0b0001 > 0,
        };
        return output;
    }

    //Flags
    pub fn set_flag(&mut self, f: Flag) {
        match f {
            Flag::Z => self.set_f(self.get_f() | 0b10000000),
            Flag::N => self.set_f(self.get_f() | 0b01000000),
            Flag::H => self.set_f(self.get_f() | 0b00100000),
            Flag::C => self.set_f(self.get_f() | 0b00010000),
        }
    }
    pub fn clear_flag(&mut self, f: Flag) {
        match f {
            Flag::Z => self.set_f(self.get_f() & 0b01111111),
            Flag::N => self.set_f(self.get_f() & 0b10111111),
            Flag::H => self.set_f(self.get_f() & 0b11011111),
            Flag::C => self.set_f(self.get_f() & 0b11101111),
        }
    }

    pub fn exec(&mut self, operands: Op, ram: &mut [u8; 0x10000]) {
        match operands {
            Op::No(instruct) => instruct(self),
            Op::U8(instruct) => instruct(self, ram[(self.pc + 1) as usize]),
            Op::U16(instruct) => instruct(
                self,
                ram[(self.pc + 2) as usize],
                ram[(self.pc + 1) as usize],
            ),
            Op::Ram(instruct) => instruct(self, ram),
            Op::RamU8(instruct) => instruct(self, ram[(self.pc + 1) as usize], ram),
            Op::RamU16(instruct) => instruct(
                self,
                ram[(self.pc + 2) as usize],
                ram[(self.pc + 1) as usize],
                ram,
            ),
        }
    }

    pub fn write(&mut self, ram: &mut [u8; 0x10000], n: u8, adr: u16) {
        match adr {
            0x0000..=0x7FFF => {}
            0xff04 => {
                ram[0xff04] = 0;
            }
            _ => {
                ram[adr as usize] = n;
            }
        }
    }

    //memory manipulation

    pub fn write_u16_to_stack(&mut self, n: u16, ram: &mut [u8; 0x10000]) {
        self.set_sp(self.get_sp().wrapping_sub(2));
        ram[self.get_sp() as usize] = n as u8;
        ram[(self.get_sp() + 1) as usize] = (n >> 8) as u8;
    }

    pub fn read_u16_from_stack(&mut self, ram: &[u8; 0x10000]) -> u16 {
        let h: u8 = ram[(self.get_sp() + 1) as usize];
        let l: u8 = ram[self.get_sp() as usize];
        let value: u16 = ((h as u16) << 8) | l as u16;
        self.set_sp(self.get_sp().wrapping_add(2));
        return value;
    }

    //ticks voodoo magic

    pub fn set_ticks(&mut self, ticks: u8) {
        self.pending_ticks = self.pending_ticks.wrapping_add(ticks);
    }

    pub fn get_ticks(&self) -> u8 {
        let temp: u8 = self.pending_ticks;
        return temp;
    }

    pub fn clear_ticks(&mut self) {
        self.pending_ticks = 0;
    }
}

pub enum Flag {
    Z,
    N,
    H,
    C,
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

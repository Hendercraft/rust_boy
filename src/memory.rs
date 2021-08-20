use crate::{Config, file_io};
use std::fmt;

// Sources:
// https://retrocomputing.stackexchange.com/questions/11732/how-does-the-gameboys-memory-bank-switching-work
// https://gbdev.io/pandocs/Memory_Map.html
// https://b13rg.github.io/Gameboy-MBC-Analysis/
pub enum MBC {
    MBC1,
    MBC2,
    MBC3,
    MBC5,
    MMM01,
    HuC1,
    HuC3,
}

impl fmt::Display for MBC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MBC::MBC1 => write!(f, "MBC1"),
            MBC::MBC2 => write!(f, "MBC2"),
            MBC::MBC3 => write!(f, "MBC3"),
            MBC::MBC5 => write!(f, "MBC5"),
            MBC::MMM01 => write!(f, "MMM01"),
            MBC::HuC1 => write!(f, "HuC1"),
            MBC::HuC3 => write!(f, "HuC3"),
        }
    }
}

pub struct CartridgeKind {
    mbc: Option<MBC>,
    ram: bool,
    battery: bool,
    timer: bool,
    rumble: bool,
}

impl fmt::Display for CartridgeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}{}{}", 
            if let Some(mbc) = &self.mbc { mbc.to_string() } else { "ROM".to_string() },
            if self.timer { " + TIMER" } else { "" }, if self.rumble { " + RUMBLE" } else { "" },
            if self.ram { " + RAM" } else { "" }, if self.battery { " + BATTERY" } else { "" },
        )
    }
}

impl CartridgeKind {
    // Create one with info from the dedicated byte
    fn from(code: u8) -> CartridgeKind {
        match code {
            0x00 | 0x08..=0x09 => CartridgeKind { 
                mbc: None,
                ram: code >= 0x08, battery: code == 0x09,
                timer: false, rumble: false,
            },
            0x01..=0x03 => CartridgeKind { 
                mbc: Some(MBC::MBC1),
                ram: code >= 0x02, battery: code == 0x03,
                timer: false, rumble: false,
            },
            0x05..=0x06 => CartridgeKind { 
                mbc: Some(MBC::MBC2),
                ram: false, battery: code == 0x06,
                timer: false, rumble: false,
            },
            0x0B..=0x0D => CartridgeKind { 
                mbc: Some(MBC::MMM01),
                ram: code >= 0x0C, battery: code == 0x0D,
                timer: false, rumble: false,
            },
            0x0F..=0x13 => CartridgeKind { 
                mbc: Some(MBC::MBC3),
                ram: code == 0x10 || code >= 0x12, battery: code <= 0x10 || code == 0x13,
                timer: code <= 0x10, rumble: false,
            },
            0x19..=0x1E => CartridgeKind { 
                mbc: Some(MBC::MBC5),
                ram: code != 0x19 && code != 0x1C, battery: code == 0x1B || code == 0x1E,
                timer: false, rumble: code >= 0x1C,
            },
            0xFE => CartridgeKind { 
                mbc: Some(MBC::HuC3),
                ram: false, battery: false,
                timer: false, rumble: false,
            },
            0xFF => CartridgeKind { 
                mbc: Some(MBC::HuC1),
                ram: true, battery: true,
                timer: false, rumble: false,
            },
            _ => panic!("Invalid cartridge type in ROM[0x0147]")
        }
    }
}

pub struct Cartridge {
    pub banks: Vec<[u8; 0x4000]>,
    pub kind: CartridgeKind,
    active_bank: u8,
}

impl Cartridge {
    pub fn new(config: &Config) -> Cartridge {
        let banks = file_io::load_rom(config);
        let cartridge_kind_code = banks[0][0x0147];
        Cartridge {
            banks,
            kind: CartridgeKind::from(cartridge_kind_code),
            active_bank: 1,
        }
    }
}

pub struct Memory {
    pub cartridge: Cartridge,
    pub ram: [u8; 0x8000],
}

impl Memory {
    pub fn new(config: &Config) -> Memory {
        Memory {
            cartridge: Cartridge::new(config),
            ram: [0; 0x8000],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // Fixed cartridge bank read
            0x0000..=0x3FFF => self.cartridge.banks[0][addr as usize],
            // Switchable cartridge bank read
            0x4000..=0x7FFF => self.cartridge.banks[self.cartridge.active_bank as usize][(addr & 0x3FFF) as usize],
            // External RAM read
            0xA000..=0xBFFF => todo!("External RAM read"),
            // Mirror of C000~DDFF
            0xE000..=0xFDFF => self.ram[((addr & 0x1FFF) | 0xC000) as usize],
            // Normal RAM read
            _ => self.ram[(addr & 0x7FFF) as usize],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            // RAM enable
            0x0000..=0x1FFF => if self.cartridge.kind.ram { todo!("RAM enable") },
            // Select lower 5 bits of the bank number
            0x2000..=0x3FFF => if let Some(mbc) = &self.cartridge.kind.mbc {
                match mbc {
                    MBC::MBC1 | MBC::MBC2 | MBC::MBC3 | MBC::HuC1 | MBC::HuC3 => {
                        let mut data_lower_5 = data & 0b0001_1111;
                        // Bank switching bug
                        if data_lower_5 == 0 {
                            data_lower_5 += 1;
                        }
                        self.cartridge.active_bank = (self.cartridge.active_bank & 0b1110_0000) | data_lower_5;
                    },
                    MBC::MBC5 => self.cartridge.active_bank = (self.cartridge.active_bank & 0b1110_0000) | data & 0b0001_1111,
                    MBC::MMM01 => todo!("Meta bank switching"),
                }
            },
            // Select bits 6 and 7 of the bank number
            0x4000..=0x5FFF => {
                // TODO: behavior when in RAM mode
                self.cartridge.active_bank = (self.cartridge.active_bank & 0b1001_1111) | ((data & 0b0000_0011) << 5);
            },
            // ROM/RAM mode select
            0x6000..=0x7FFF => todo!("ROM/RAM mode select"),
            // Mirror of C000~DDFF
            0xE000..=0xFDFF => self.ram[((addr & 0x1FFF) | 0xC000) as usize] = data,
            // Special behavior of 0xFF04
            0xFF04 => self.ram[0x7F04] = 0,
            // Normal RAM writes
            _ => self.ram[(addr & 0x7FFF) as usize] = data,
        }
    }
}

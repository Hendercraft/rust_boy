use std::fs;
use std::io;

use crate::{hardware::Cpu, Config};

pub fn load_rom(config: &Config) -> Vec<Vec<u8>> {
    let contents = fs::read(&config.rom_path).expect("Something went wrong reading the file");
    let mut rom = Vec::new();
    let banks = contents.len() / 0x4000;
    for i in 0..banks {
        rom.push(Vec::new());
        for j in 0..0x4000 {
            rom[i].push(contents[((i * 0x4000) + j) as usize]);
        }
    }

    if config.debug >= 1 {
        println!("Banks: {}", rom.len());
        println!("Bank size: {}", rom[0].len());
    }

    rom
}

pub fn init_ram(rom: &Vec<Vec<u8>>) -> [u8; 0x10000] {
    let mut ram: [u8; 0x10000] = [0; 0x10000];
    for i in 0..0x4000 {
        ram[i] = rom[0][i];
    }
    for i in 0x4000..0x8000 {
        ram[i] = rom[1][(i - 0x4000) as usize];
    }
    ram
}

pub fn create_savestate(config: &Config, cpu: &Cpu, ram: &[u8; 0x10000]) {
    let mut buffer = bincode::serialize(&cpu).unwrap();
    buffer.append(&mut ram.to_vec());

    let savestate_path = get_savestate_path(config);

    // Create saves folder if it doesn't exist yet
    fs::create_dir_all("saves").expect("Unable to create saves folder");

    // Create savestate
    fs::write(savestate_path, buffer).expect("Unable to create savestate");
}

pub fn load_savestate(config: &Config, cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    let savestate_path = get_savestate_path(config);

    // Open and read savestate file. The panics should eventually be replaced with an on-screen message.
    let buffer = fs::read(&savestate_path).unwrap_or_else(|e| match e.kind() {
        io::ErrorKind::NotFound => panic!("Savestate {} not found", savestate_path),
        io::ErrorKind::PermissionDenied => panic!("Permission denied"),
        _ => panic!("Unable to read savestate file"),
    });

    let buffer_len = buffer.len();
    // Restore RAM
    ram.copy_from_slice(&buffer[buffer_len - 0x10000..]);

    // Restore CPU
    *cpu = bincode::deserialize(&buffer[..buffer_len - 0x10000])
        .expect("Unable to decode CPU data, did you edit the savestate file?");
}

fn get_savestate_path(config: &Config) -> String {
    let rom_name = config
        .rom_path
        .file_stem()
        .expect("Unable to extract rom name")
        .to_str()
        .expect("File name doesn't contain valid Unicode");

    format!("saves/{}_0.savestate", rom_name)
}

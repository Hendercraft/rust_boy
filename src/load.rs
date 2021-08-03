use std::fs;

pub fn load(path: String) -> Vec<Vec<u8>> {
    let contents = fs::read(path).expect("Something went wrong reading the file");
    let mut rom = Vec::new();
    let banks = contents.len()/0x4000;
    for i in 0..banks {
        rom.push(Vec::new());
        for j in 0..0x4000{
            rom[i].push(contents[((i * 0x4000) + j) as usize]);
        }
    }
    println!("Banks:{}",rom.len());
    println!("Bank size:{}",rom[0].len());
    return rom;

}

pub fn init_ram(rom: &Vec<Vec<u8>>) -> [u8; 0x10000]{
    let mut ram: [u8; 0x10000] = [0; 0x10000];
    for i in 0..0x4000{
        ram[i] = rom[0][i];
    }
    for i in 0x4000..0x8000{
        ram[i] = rom[1][(i-0x4000) as usize];
    }
    return ram;
}

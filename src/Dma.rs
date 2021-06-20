pub fn update_dma(ram: &mut [u8;0x10000]){
    if ram[0xff46] != 0 {
        //println!("/!\\ DMA HAS OCCURRED");
        let source:u16 = (ram[0xff46] as u16) << 8;
        let dest:u16 = 0xFE00;
        for i in 0..0xA0{
            ram[(dest + i) as usize] = ram[(source + i) as usize];
        }
        ram[0xff46] = 0;
    }
}

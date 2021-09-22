use crate::memory::Memory;

pub fn update_dma(mem: &mut Memory) {
    if mem.read(0xff46) != 0 {
        //println!("/!\\ DMA HAS OCCURRED");
        let source: u16 = (mem.read(0xff46) as u16) << 8;
        let dest: u16 = 0xFE00;
        for i in 0..0xA0 {
            mem.write(dest + i, mem.read(source + i));
        }
        mem.write(0xff46, 0);
    }
}

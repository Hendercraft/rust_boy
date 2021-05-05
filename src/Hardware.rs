#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

struct Flags{
    Z : bool,
    N : bool,
    H : bool,
    C : bool,
}

pub struct Cpu{
    a : u8,
    f : u8,
    b : u8,
    c : u8,
    d : u8,
    e : u8,
    h : u8,
    l : u8,
    sp : u16,
    pc : u16,
    //flags : Flags,
    instruc : Vec<Instruc>
}

impl Cpu {

    fn get_u16 (h: u8 ,l: u8) -> u16{return 8;}

    fn set_u16 (h: u8,l: u8,n: u16){}

    fn get_flags (&self) -> Flags{
        let temp = self.f >> 4;
        let mut output : Flags = Flags{
            Z : temp & 0b1000 > 0, //get upper
            N : temp & 0b0100 > 0,
            H : temp & 0b0010 > 0,
            C : temp & 0b0001 > 0,
        };
        return output;
    }

    fn set_flags (f: u8){}

    fn fetch(&self,i: u8) -> &Instruc {&self.instruc.get(i as usize).unwrap()}

    fn exec<'a>(i :&Instruc){}


}

pub struct Gpu{
    screen : [[u8;160];144],
    matrix : [[u8;255];255],
    line : u8
}

impl Gpu{
    fn pushToScreen(&mut self){
        //push matrix to SDL2
        self.line = 0
    }

    fn fetchLine(&mut self, ram: &mut[u8]){
        //fetch the BG and window's content from RAM
        let tileLine: u8 = self.line / 8;
        let lineOffset: u8 = self.line % 8;
        let mut bgTileStart:u16;
        if ram[0xFF40] & 0b00001000 > 0 {
            bgTileStart = 0x9C00 + 32*(tileLine as u16);
        }else{
            bgTileStart = 0x9800 + 32*(tileLine as u16);
        }
        let mut tileAdress:u16;
        let mut lineAdress:u16;
        for i in 0..32 {
            tileAdress = bgTileStart + i*32;
            lineAdress = tileAdress + (lineOffset as u16) *2;
            //TODO: transformation via les palettes
            for j in 0..2{
                self.matrix[(32*i + j*4) as usize][self.line as usize] = ram[(lineAdress + j) as usize] & 0b11000000 >> 6;
                self.matrix[(32*i + j*4 + 1) as usize][self.line as usize] = ram[(lineAdress + j) as usize] & 0b00110000 >> 4;
                self.matrix[(32*i + j*4 + 2) as usize][self.line as usize] = ram[(lineAdress + j) as usize] & 0b00001100 >> 2;
                self.matrix[(32*i + j*4 + 3) as usize][self.line as usize] = ram[(lineAdress + j) as usize] & 0b00000011;
            }
        }
        self.line += 1;
        if self.line > 255 {
            self.pushToScreen();
            self.line = 0;
        }
    }

    fn getBgWinTile(n:u8, ram: &[u8]) -> u8{//return the adress of the tile in the memory
        let startIndex:u16;
        if ram[0xF40] & 0b00010000 > 0 {
            startIndex = 0x8000;
        }else{
            startIndex = 0x8800;
        }
        ram[(startIndex + 16*(n as u16)) as usize]
    }

}

pub struct Instruc {
    pub n : u16,
    pub name : String,
    pub desc : String,
    pub argc : u8,
    pub tics : u8,
    pub exec : fn(&mut Cpu),
}

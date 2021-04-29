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
        self.line += 1;
        if self.line > 144 {
            self.pushToScreen();
            self.line = 0;
        }
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
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::num::Wrapping;

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
    mie : bool,
    //flags : Flags,
    instructs: Vec<Instruct>
}

impl Cpu {
    //getter u8
    pub fn get_a (&self) -> u8 {self.a}
    pub fn get_f (&self) -> u8 {self.f}
    pub fn get_b (&self) -> u8 {self.b}
    pub fn get_c (&self) -> u8 {self.c}
    pub fn get_d (&self) -> u8 {self.d}
    pub fn get_e (&self) -> u8 {self.e}
    pub fn get_h (&self) -> u8 {self.h}
    pub fn get_l (&self) -> u8 {self.l}
    //getter u16
    pub fn get_af (&self) ->u16 {Cpu::get_u16(&self.a,&self.f)}
    pub fn get_bc (&self) ->u16 {Cpu::get_u16(&self.b,&self.c)}
    pub fn get_de (&self) ->u16 {Cpu::get_u16(&self.d,&self.e)}
    pub fn get_hl (&self) ->u16 {Cpu::get_u16(&self.h,&self.l)}
    pub fn get_sp (&self) ->u16 {self.sp}
    pub fn get_pc (&self) ->u16 {self.pc}

    fn get_u16 (h: &u8 ,l: &u8) -> u16{
        ((*h as u16) << 8) | *l as u16
    }

    //setter u8
    pub fn set_a (&mut self, n: u8){self.a = n;}
    pub fn set_f (&mut self, n: u8){self.f = n;}
    pub fn set_b (&mut self, n: u8){self.b = n;}
    pub fn set_c (&mut self, n: u8){self.c = n;}
    pub fn set_d (&mut self, n: u8){self.d = n;}
    pub fn set_e (&mut self, n: u8){self.e = n;}
    pub fn set_h (&mut self, n: u8){self.h = n;}
    pub fn set_l (&mut self, n: u8){self.l = n;}

    //setter u16
    pub fn set_af (&mut self, n : u16) {Cpu::set_u16(&mut self.a,&mut self.f,n);}
    pub fn set_bc (&mut self, n : u16) {Cpu::set_u16(&mut self.b,&mut self.c,n);}
    pub fn set_de (&mut self, n : u16) {Cpu::set_u16(&mut self.d,&mut self.e,n);}
    pub fn set_hl (&mut self, n : u16) {Cpu::set_u16(&mut self.h, &mut self.l,n);}
    pub fn set_sp (&mut self, n : u16) {self.sp = n;}
    pub fn set_pc (&mut self, n : u16) {self.pc = n;}

    fn set_u16 (h: &mut u8, l: &mut u8, n: u16){
        *h = (n >> 8) as u8;
        *l = n as u8;
    }


    pub(crate) fn inc_u16(h : &mut u8, l: &mut u8){
        let n = Cpu::get_u16(h,l) + 1;
        Cpu::set_u16(h,l,n);
    }



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

    fn fetch(&self,i: u8) -> &Instruct {&self.instructs.get(i as usize).unwrap()}

    fn exec<'a>(i :&Instruct){}


}

pub struct Gpu{
    pub screen : [[u8;144];160],
    pub bgMatrix : [[u8;256];256],
    pub windowMatrix : [[u8;256];256],
    pub line : u8
}

impl Gpu{
    fn pushToScreen(&mut self){
        //push matrix to SDL2
        self.line = 0
    }


    fn getTileMethod(&self, ram: &[u8;0xffff]) -> u16{
        if ram[0xff40] & 0b00010000 > 0{
            println!("0x8000");
            return 0x8000;
        }else{
            println!("0x8800");
            return 0x8800;
        }
    }

    fn getBgMapIndex(&self, ram: &[u8;0xffff]) -> u16{
        if ram[0xff40] & 0b00001000 > 0{
            println!("0x9c00");
            return 0x9c00;
        }else{
            println!("0x9800");
            return 0x9800;
        }
    }

    fn getWindowMapIndex(&self, ram: &[u8;0xffff]) -> u16{
        if ram[0xff40] & 0b01000000 > 0{
            println!("0x9c00");
            return 0x9c00;
        }else{
            println!("0x9800");
            return 0x9800;
        }
    }

    fn getTile(&self, method:u16, mut index:u8, ram: &[u8;0xffff]) -> u16{
        if method == 0x8000{
            return 0x8000 + (index as u16)*16;
        }else{
            if index > 127{
                index -= 128;
                return 0x8800 + (index as u16)*16;
            }else{
                return 0x9000 + (index as u16)*16;
            }
        }
    }

    fn displayTile(&mut self, isWin:bool, x:u16, y:u16, location:u16,ram: &[u8;0xffff]){
        let &mut mat;
        if isWin{
            mat = &mut self.windowMatrix;
        }else{
            mat = &mut self.bgMatrix;
        }
        let origin_x = x*8;
        let origin_y = y*8;
        for i in 0..8{
            let mut value:u8 = 0b10000000;
            for j in 0..7{
                mat[(origin_x+j) as usize][(origin_y+i) as usize] = (ram[(location + 2*(i as u16)) as usize] & value) >> 7-j | (ram[(location + 2*(i as u16) + 1) as usize] & value) >> 6-j;
                value = value >> 1;
            }
            mat[(origin_x+7) as usize][(origin_y+i) as usize] = (ram[(location + 2*(i as u16)) as usize] & value) | (ram[(location + 2*(i as u16) + 1) as usize] & value) << 1;

        }
        println!("Tile adress: 0x{:x}",location);
        println!("Tile coords: x={} y={}",x,y);
        println!("__________________________");
    }

    pub fn buildBG(&mut self, ram: &[u8;0xffff]){
        let mut n:u16 = 0;

        let index = self.getBgMapIndex(&ram);
        let method:u16 = self.getTileMethod(&ram);

        for i in 0..32{
            for j in 0..32{
                println!("Map adress: 0x{:x}",index+n);
                self.displayTile(false, j as u16, i as u16, self.getTile(method, ram[(index+n) as usize], &ram), &ram);
                n += 1;
            }
        }

    }

    pub fn buildWindow(&mut self, ram: &[u8;0xffff]){
        let mut n:u16 = 0;

        let index = self.getWindowMapIndex(&ram);
        let method:u16 = self.getTileMethod(&ram);

        for i in 0..32{
            for j in 0..32{
                println!("Map adress: 0x{:x}",index+n);
                self.displayTile(true, j as u16, i as u16, self.getTile(method, ram[(index+n) as usize], &ram), &ram);
                n += 1;
            }
        }

    }

    pub fn pushLine(&mut self, ram: &[u8;0xffff]){
        let scrollX:u8 = ram[0xff43];
        let scrollY:u8 = ram[0xff42];
        for i in 0..160{
            self.screen[i as usize][self.line as usize] = self.bgMatrix[(scrollX.wrapping_add(i)) as usize][(scrollY.wrapping_add(self.line)) as usize];
        }
        self.line += 1;
        if self.line == 144{

            self.line = 0;
        }

    }


}

pub enum Op{
    no(fn(&mut Cpu)),
    u8toCpu(fn(&mut Cpu,u8)),
    u16toCpu(fn(&mut Cpu,u8,u8)),//High, low
}

pub struct Instruct {
    pub n : u16,
    pub name : String,
    pub desc : String,
    pub argc : u8,
    pub tics : u8,
    pub exec : Op,
}



impl Instruct {

    pub fn build_instruct(n: u16, name : String, desc : String, argc : u8, tics : u8, exec : Op) -> Instruct {
        Instruct {n,name,desc,argc,tics,exec}
    }







}

use crate::Gui::Gui as Gui;
use sdl2::event::Event;
use sdl2::keyboard::*;

pub struct Controls{
    pub up: u8,
    pub down: u8,
    pub left: u8,
    pub right: u8,
    pub a: u8,
    pub b: u8,
    pub select: u8,
    pub start: u8,
}

impl Controls{
    pub fn getKeyboard(&mut self, gui: &mut Gui){
        self.up = 0;
        self.down = 0;
        self.left = 0;
        self.right = 0;
        self.a = 0;
        self.b = 0;
        self.select = 0;
        self.start = 0;
        let mut iterator = gui.events.keyboard_state();
        for scancode in iterator.pressed_scancodes(){
            match scancode{
                Scancode::Up =>{
                    //println!("up");
                    self.up = 1;
                },
                Scancode::Down =>{
                    //println!("down");
                    self.down = 1;
                },
                Scancode::Left =>{
                    //println!("left");
                    self.left = 1;
                },
                Scancode::Right =>{
                    //println!("right");
                    self.right = 1;
                },
                Scancode::Q =>{
                    //println!("a");
                    self.a = 1;
                },
                Scancode::W =>{
                    //println!("b");
                    self.b = 1;
                },
                Scancode::Return =>{
                    //println!("start");
                    self.start = 1;
                },
                Scancode::Backspace =>{
                    //println!("select");
                    self.select = 1;
                },
                _ => {}
            }
        }
    }

    pub fn updateRam(&self, ram: &mut [u8;0x10000]){
        let mut n = ram[0xff00];
        if n & 0b00100000 > 0{
            //cross
            n = 0b00100000 +
            (self.down << 3) +
            (self.up << 2) +
            (self.left << 1) +
            (self.right);
        }else{
            //buttons
            n = 0b00010000 +
            (self.start << 3) +
            (self.select << 2) +
            (self.b << 1) +
            (self.a);
        }
        ram[0xff00] = n;
    }
}
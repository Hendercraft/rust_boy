use crate::gui::Gui;
use sdl2::keyboard::*;

pub struct Controls {
    pub up: u8,
    pub down: u8,
    pub left: u8,
    pub right: u8,
    pub a: u8,
    pub b: u8,
    pub select: u8,
    pub start: u8,
}

impl Controls {
    pub fn get_keyboard(&mut self, gui: &mut Gui) {
        self.up = 1;
        self.down = 1;
        self.left = 1;
        self.right = 1;
        self.a = 1;
        self.b = 1;
        self.select = 1;
        self.start = 1;
        let iterator = gui.events.keyboard_state();
        for scancode in iterator.pressed_scancodes() {
            match scancode {
                Scancode::Up => {
                    //println!("up");
                    self.up = 0;
                }
                Scancode::Down => {
                    //println!("down");
                    self.down = 0;
                }
                Scancode::Left => {
                    //println!("left");
                    self.left = 0;
                }
                Scancode::Right => {
                    //println!("right");
                    self.right = 0;
                }
                Scancode::Q => {
                    //println!("a");
                    self.a = 0;
                }
                Scancode::W => {
                    //println!("b");
                    self.b = 0;
                }
                Scancode::Return => {
                    //println!("start");
                    self.start = 0;
                }
                Scancode::Backspace => {
                    //println!("select");
                    self.select = 0;
                }
                _ => {}
            }
        }
    }

    pub fn update_ram(&self, ram: &mut [u8; 0x10000]) {
        let mut n = ram[0xff00];
        if n & 0b00100000 > 0 {
            //cross
            n = 0b11100000 | (self.down << 3) | (self.up << 2) | (self.left << 1) | (self.right);
        } else {
            //buttons
            n = 0b11010000 | (self.start << 3) | (self.select << 2) | (self.b << 1) | (self.a);
        }
        ram[0xff00] = n;
    }
}

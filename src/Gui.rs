#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::Rect;

struct Gui{
    context : Sdl,
    video : VideoSubsystem,
    canvas : render::Canvas,
}

impl Gui{
    fn init(&mut self) -> Gui {
        let self.context = sdl2::init().unwrap();
    }
}

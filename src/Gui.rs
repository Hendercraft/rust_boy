#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use sdl2::{Sdl,VideoSubsystem,EventPump};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::rect::{Rect, Point};
use sdl2::video::WindowContext;


pub struct Gui{
    pub canvas : WindowCanvas,
    pub events : EventPump
}

impl Gui{
    pub fn new() -> Gui{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();


        let window = video_subsystem.window("Rustboy", 480, 432)//160X144
            .position_centered()
            .opengl()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window.into_canvas().accelerated().present_vsync().build()
            .expect("could not make a canvas");
        canvas.set_scale(3.0,3.0);
        canvas.clear();
        let event_pump = sdl_context.event_pump().unwrap();
        Gui{
            //context: sdl_context,
            //video: video_subsystem,
            canvas: canvas,
            events: event_pump
        }
    }

    pub fn clear(&mut self){
        self.canvas.set_draw_color((255,255,255));
        self.canvas.clear();
    }

    pub fn update(&mut self) -> bool{
        if self.events.poll_iter().any(|i| matches!(i, Event::Quit {..})) {
            false
        }
        else {
            self.canvas.present();
            true
        }
    }

    pub fn pushMatrix(&mut self, mat: &[[u8;144];160], texture: &mut Texture){
        let mut pixel_data : [u8;69120] = [0;69120];//BGR
        let mut offset : u32 = 0;
        let mut r : u8 = 0;
        let mut g : u8 = 0;
        let mut b : u8 = 0;
        for i in 0..mat.len(){
            for j in 0..mat[i].len(){
                //println!("{}",mat[i][j]);
                match mat[i][j]{
                    0 => {r=224;g=248;b=208;},
                    1 => {r=136;g=192;b=112;},
                    2 => {r=52;g=104;b=86;},
                    _ => {r=8;g=24;b=32;},
                }
                offset = (j as u32) * 480 + (i as u32) * 3;
                pixel_data[(offset)as usize] = r;
                pixel_data[(offset+1)as usize] = g;
                pixel_data[(offset+2)as usize] = b;
                offset += 3;
            }
        }


        texture.update(None,&pixel_data, 480 as usize).expect("Couldn't update texture");
        self.canvas.copy(&texture, None, None);


    }

}

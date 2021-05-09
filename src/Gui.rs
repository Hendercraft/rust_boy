#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use sdl2::{Sdl,VideoSubsystem,EventPump};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::{Rect, Point};


pub struct Gui{
    //context : Sdl,
    //video : VideoSubsystem,
    canvas : WindowCanvas,
    events : EventPump
}

impl Gui{
    pub fn new() -> Gui{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();


        let window = video_subsystem.window("Rustboy", 512, 512)
            .position_centered()
            .opengl()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window.into_canvas().accelerated().build()
            .expect("could not make a canvas");
        canvas.set_scale(2.0,2.0);
        canvas.set_draw_color((255,255,255));
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
        for event in self.events.poll_iter(){
            match event{
                Event::Quit {..} => {return false;},
                _ => {}
            }
        }
        self.canvas.present();
        return true;
    }

    pub fn pushMatrix(&mut self, mat: &[[u8;256];256]){
        for i in 0..256{
            for j in 0..256{
                //println!("{}",mat[i][j]);
                match mat[i][j]{
                    0 => {self.canvas.set_draw_color((255,255,255))},
                    1 => {self.canvas.set_draw_color((170,170,170))},
                    2 => {self.canvas.set_draw_color((84,84,84))},
                    _ => {self.canvas.set_draw_color((0,0,0))},
                }
                self.canvas.draw_point(Point::new(i as i32,j as i32));
            }
        }
    }
}

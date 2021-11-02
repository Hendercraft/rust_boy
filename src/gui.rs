use sdl2::event::Event;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;

use crate::Config;

pub struct Gui {
    pub canvas: WindowCanvas,
    pub events: EventPump,
}

impl Gui {
    pub fn new(config: &Config) -> Gui {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let mut window = video_subsystem.window("Rustboy", 480, 432); //160X144

        if config.full_screen {
            window.fullscreen();
        }

        let window = window
            .position_centered()
            .opengl()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .expect("could not make a canvas");

        canvas
            .set_scale(3.0, 3.0)
            .expect("Could not set canvas scale");

        canvas.clear();

        let event_pump = sdl_context.event_pump().unwrap();
        Gui {
            //context: sdl_context,
            //video: video_subsystem,
            canvas: canvas,
            events: event_pump,
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color((255, 255, 255));
        self.canvas.clear();
    }

    pub fn update(&mut self) -> bool {
        if self
            .events
            .poll_iter()
            .any(|i| matches!(i, Event::Quit {..}))
        {
            false
        } else {
            self.canvas.present();
            true
        }
    }

    pub fn push_matrix(&mut self, mat: &[[u8; 144]; 160], texture: &mut Texture) {
        let mut pixel_data: [u8; 69120] = [0; 69120]; //BGR
        let mut offset: u32;
        let mut r: u8;
        let mut g: u8;
        let mut b: u8;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                //println!("{}",mat[i][j]);
                match mat[i][j] {
                    0 => {
                        r = 224;
                        g = 248;
                        b = 208;
                    }
                    1 => {
                        r = 136;
                        g = 192;
                        b = 112;
                    }
                    2 => {
                        r = 52;
                        g = 104;
                        b = 86;
                    }
                    _ => {
                        r = 8;
                        g = 24;
                        b = 32;
                    }
                }

                offset = (j as u32) * 480 + (i as u32) * 3;
                pixel_data[(offset) as usize] = r;
                pixel_data[(offset + 1) as usize] = g;
                pixel_data[(offset + 2) as usize] = b;
            }
        }

        texture
            .update(None, &pixel_data, 480 as usize)
            .expect("Couldn't update texture");
        self.canvas
            .copy(&texture, None, None)
            .expect("Couldn't copy texture on canvas");
    }
}

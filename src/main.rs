mod controls;
mod dma;
mod file_io;
mod gui;
mod hardware;
mod instructions;
mod interrupts;
mod master;
mod timer;
const PX_TRANSFER: u8 = 2;

use sdl2::gfx::framerate::FPSManager;
use sdl2::pixels::PixelFormatEnum;
use std::cmp;
use std::path::Path;

#[macro_use]
extern crate clap;

pub struct Config<'a> {
    pub rom_path: &'a Path,
    pub debug: u32,
    pub full_screen: bool,
    pub framerate: u32,
}

fn main() {
    let matches = clap_app!(rust_boy =>
        (version: crate_version!())
        (author: crate_authors!(", "))
        (about: crate_description!())
        (@arg ROM: +required "Sets the ROM to use")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@arg fullscreen: -F --fullscreen "Runs the emulator in full screen mode")
        (@arg framerate: -f --framerate +takes_value "Sets FPS. Default is 60, use 0 for unlimited. Note: this changes the game speed as well")
    )
    .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required
    let rom_path = Path::new(matches.value_of("ROM").unwrap());

    // Vary the output based on how many times the user used the "debug" flag
    // (i.e. 'rust_boy -d -d -d' or 'rust_boy -ddd' vs 'rust_boy -d'
    let debug = cmp::min(matches.occurrences_of("debug") as u32, 2);

    let framerate = value_t!(matches, "framerate", u32).unwrap_or_else(|e| match e.kind {
        clap::ErrorKind::ArgumentNotFound => 60,
        clap::ErrorKind::ValueValidation => {
            println!(
                "Warning: \"{}\" is not a valid FPS value, defaulted to 60.",
                matches.value_of("framerate").unwrap()
            );
            60
        }
        _ => e.exit(),
    });

    let config = Config {
        rom_path,
        debug,
        full_screen: matches.is_present("fullscreen"),
        framerate,
    };

    if config.debug >= 1 {
        println!("Debug mode enabled, level: {}", config.debug);
    }

    // Initialize both ROM and RAM
    let rom = file_io::load_rom(&config);
    let mut ram = file_io::init_ram(&rom);

    let mut controls: controls::Controls = controls::Controls {
        up: 0,
        down: 0,
        left: 0,
        right: 0,
        a: 0,
        b: 0,
        select: 0,
        start: 0,
    };

    let mut window: gui::Gui = gui::Gui::new(&config);
    let creator = window.canvas.texture_creator();
    let mut texture = creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 160, 144)
        .expect("Couldn't create texture");

    let mut gpu: hardware::Gpu = hardware::Gpu {
        screen: [[0; 144]; 160],
        bg_matrix: [[0; 256]; 256],
        window_matrix: [[0; 256]; 256],
        sprite_matrix: [[0; 256]; 256],
        line: 0,
    };

    let mut cpu: hardware::Cpu = hardware::Cpu {
        a: 0,
        f: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        sp: 0,     //0xfffe, // default value
        pc: 0x100, //default valueS
        mie: true,
        pending_mie: None,
        pending_ticks: 0,
    };

    let mut timer: timer::Timer = timer::Timer {
        divider_ticks: 0, //update every 256
        division: 0,
        timer_ticks: 0, //Update every division
        timer_enb: false,
    };

    let mut master: master::Master = master::Master {
        nb_steps: 0,
        tick: 0,
        mode: PX_TRANSFER,
        previous_mode: PX_TRANSFER,
        step_by_step: false,
        line_by_line: false,
        screen_by_screen: false,
        log: false,
    };
    //ram[0xff05] = 255;
    //ram[0xffff] = 0;
    //ram[0xff0f] = 0;
    //ram[0xFF46] = 0;
    //ram[0xFF41] = 255;
    //ram[0xFF45] = 1;

    let mut frm = FPSManager::new();
    if config.framerate > 0 {
        frm.set_framerate(config.framerate)
            .expect("Couldn't set framerate");
    }

    while window.update() {
        window.clear();
        controls.get_keyboard(&config, &mut cpu, &mut ram, &mut window);
        controls.update_ram(&mut ram);
        window.push_matrix(&gpu.screen, &mut texture);
        master.screen(&mut cpu, &mut gpu, &mut timer, &mut controls, &mut ram);
        gpu.build_bg(&ram);
        gpu.build_window(&ram);
        gpu.build_sprite(&ram);
        if config.framerate > 0 {
            frm.delay();
        }
    }
}

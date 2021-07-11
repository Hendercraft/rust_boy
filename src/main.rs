mod controls;
mod dma;
mod gui;
mod hardware;
mod instruct_array;
mod interrupts;
mod master;
mod timer;
const PX_TRANSFER: u8 = 2;

use sdl2::pixels::PixelFormatEnum;
use std::fs;

fn load(path: String) -> [u8; 0x10000] {
    let contents = fs::read(path).expect("Something went wrong reading the file");
    let mut ram: [u8; 0x10000] = [0; 0x10000];
    for i in 0..contents.len() {
        ram[i] = contents[i];
    }
    return ram;
}

fn main() {
    let mut ram = load(String::from("rom.gb"));

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

    let mut window: gui::Gui = gui::Gui::new();
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
        //flags : Flags,
        instructs: instruct_array::create_operations(),
    };

    let mut timer: timer::Timer = timer::Timer {
        divider_ticks: 0, //update every 256
        division: 0,
        timer_ticks: 0, //Update every division
        timer_enb: false,
    };

    let mut master: master::Master = master::Master {
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

    while window.update() {
        window.clear();
        controls.get_keyboard(&mut window);
        controls.update_ram(&mut ram);
        window.push_matrix(&gpu.screen, &mut texture);
        master.screen(&mut cpu, &mut gpu, &mut timer, &mut controls, &mut ram);
        gpu.build_bg(&ram);
        gpu.build_window(&ram);
        gpu.build_sprite(&ram);
    }
}

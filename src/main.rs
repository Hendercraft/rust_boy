#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod Hardware;
mod InstrucArr;
mod Gui;
mod Controls;
mod Interrupts;
mod Timer;
mod Master;


use std::fs;


fn load(path: String) -> [u8;0x10000]{
    let contents = fs::read(path)
        .expect("Something went wrong reading the file");
    let mut ram : [u8;0x10000] = [0;0x10000];
    for i in 0..contents.len(){
        ram[i] = contents[i];
    }
    return ram;
}

fn main(){

    let mut ram = load(String::from("rom.gb"));


    let mut controls: Controls::Controls = Controls::Controls{
        up: 0,
        down: 0,
        left: 0,
        right: 0,
        a: 0,
        b: 0,
        select: 0,
        start: 0,
    };

    let mut window: Gui::Gui = Gui::Gui::new();

    let mut gpu:Hardware::Gpu = Hardware::Gpu{
        screen : [[0;144];160],
        bgMatrix : [[0;256];256],
        windowMatrix : [[0;256];256],
        line : 0
    };

    let mut cpu:Hardware::Cpu = Hardware::Cpu{
        a : 0,
        f : 0,
        b : 0,
        c : 0,
        d : 0,
        e : 0,
        h : 0,
        l : 0,
        sp : 0x0000, //0xfffe, // default value
        pc : 0xFF, //default value
        mie : true,
        //flags : Flags,
        instructs: InstrucArr::createOperations(),
    };

    let mut timer: Timer::Timer = Timer::Timer{
        divider_ticks : 0,//update every 256
        division : 0,
        timer_ticks : 0, //Update every division
        timer_enb : false
    };


    let mut master: Master::Master = Master::Master{
        tick: 0,
        step_by_step: true,
        line_by_line: false,
        screen_by_screen: false,
    };
    gpu.buildBG(&ram);
    gpu.buildWindow(&ram);
    for i in 0..160{
        gpu.pushLine(&ram);
    }
    //ram[0xff05] = 255;
    //ram[0xffff] = ram[0xffff] | 0b100;

    while window.update(){
        window.clear();
        controls.getKeyboard(&mut window);
        controls.updateRam(&mut ram);
        window.pushMatrix(&gpu.screen);
        master.screen(&mut cpu, &mut gpu, &mut timer, &mut ram);

    }


}

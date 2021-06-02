#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod Hardware;
mod InstrucArr;
mod Gui;
mod Controls;
mod Interrupts;
mod Clock;
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

    let mut ram = load(String::from("dump.dmp"));


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
        sp : 0,
        pc : 0,
        mie : true,
        //flags : Flags,
        instructs: InstrucArr::createOperations(),
    };


    let mut master: Master::Master = Master::Master{
        op_count: 0,
        step_by_step: false,
        screen_by_screen: false,
    };
    gpu.buildBG(&ram);
    gpu.buildWindow(&ram);
    for i in 0..160{
        gpu.pushLine(&ram);
    }

    while window.update(){
        ram[0xff00] = 0b00010000;
        window.clear();
        controls.getKeyboard(&mut window);
        controls.updateRam(&mut ram);
        window.pushMatrix(&gpu.screen);
        master.screen(&mut cpu, &mut gpu, &mut ram);

    }


}

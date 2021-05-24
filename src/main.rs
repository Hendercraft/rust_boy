#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod Hardware;
mod InstrucArr;
mod Gui;
mod Controls;
mod Interrupts;

use std::fs;
fn main(){


    let contents = fs::read("dump.dmp")
        .expect("Something went wrong reading the file");
    let mut ram : [u8;0xffff] = [0;0xffff];
    for i in 0..0xffff{
        ram[i] = contents[i];
    }

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
    gpu.buildBG(&ram);
    gpu.buildWindow(&ram);
    for i in 0..160{
        gpu.pushLine(&ram);
    }

    while window.update(){
        window.clear();
        controls.getKeyboard(&mut window);
        window.pushMatrix(&gpu.screen);
    }




}

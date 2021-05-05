#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod Hardware;
mod InstrucArr;
mod Gui;
use rand::Rng;

fn main(){
    let mut rng = rand::thread_rng();
    let mut window: Gui::Gui = Gui::Gui::new();
    let mut maxiMatrix: [[u8;255];255] = [[0;255];255];
    while window.update(){
        for i in 0..255{
            for j in 0..255{
                maxiMatrix[i][j] = rng.gen_range(0..4);

            }
        }
        window.clear();
        window.pushMatrix(&maxiMatrix);
    }
}

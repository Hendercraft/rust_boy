use crate::{Hardware, Interrupts, Controls, InstrucArr, Timer, Dma};
use std::io::{stdin, stdout, Read, Write};
const H_BLANK: u8 = 0;
const V_BLANK: u8 = 1;
const PX_TRANSFER: u8 = 2;

pub struct Master{
pub tick: u64,
pub mode: u8,
pub previous_mode: u8,
pub step_by_step: bool,
pub line_by_line: bool,
pub screen_by_screen: bool,

}

impl Master{
    pub fn step(&mut self, cpu: &mut Hardware::Cpu, gpu: &mut Hardware::Gpu, timer: &mut Timer::Timer,controls: &mut Controls::Controls, ram: &mut [u8;0x10000]){
        //Check for interrupts, if none juste add 1 to PC
        Interrupts::interrupt_check(cpu,ram);
        let instruct : &Hardware::Instruct = cpu.fetch(ram[cpu.get_pc() as usize]);
        let argc:u8 = instruct.argc;

        self.maxi_debug_print(&cpu,&gpu,&timer,&ram,&controls,&instruct);

        self.tick = self.tick.wrapping_add(instruct.ticks as u64);

        timer.update(instruct.ticks, ram);

        controls.updateRam(ram);

        cpu.exec(instruct.n,ram);

        cpu.set_pc(cpu.get_pc() + (argc as u16) + 1);

        Dma::update_dma(ram);



    }

    pub fn screen(&mut self, cpu: &mut Hardware::Cpu, gpu: &mut Hardware::Gpu, timer: &mut Timer::Timer, controls: &mut Controls::Controls, ram: &mut [u8;0x10000]){
        for i in 0..144{
            while self.tick < 114{
                if self.tick > 63{
                    self.mode = H_BLANK;
                }else{
                    self.mode = PX_TRANSFER;
                }
                print!("{esc}c", esc = 27 as char);
                println!("SCREEN STATE__________________________________");
                println!("State: Printing");
                println!("Line: {}",i);
                println!("Mode: {}",self.mode);
                println!(" ");
                self.step(cpu, gpu, timer,controls, ram);
                self.lcd_stat(i, ram);
                if self.step_by_step{
                    wait();
                }

            }
            self.tick = 0;
            gpu.pushLine(ram);
            if self.line_by_line {
                wait();
            }
        }

        ram[0xFF0F] = ram[0xFF0F] | 0b1;
        self.mode = V_BLANK;

        for j in 0..10{
            while self.tick < 114{
                print!("{esc}c", esc = 27 as char);
                println!("SCREEN STATE__________________________________");
                println!("State: V-Blank");
                println!("Mode: {}",self.mode);
                println!(" ");
                self.step(cpu, gpu, timer,controls, ram);
                self.lcd_stat(254, ram);
                if self.step_by_step {
                    wait();
                }
            }
            self.tick = 0;
            if self.line_by_line {
                wait();
            }
        }

        if self.screen_by_screen {
            wait();
        }
    }


    pub fn maxi_debug_print(&self, cpu: &Hardware::Cpu, gpu: &Hardware::Gpu, timer: &Timer::Timer, ram: &[u8;0x10000],controls: &Controls::Controls, instruc : &Hardware::Instruct){
        println!("OPERATION____________________________________");
        println!("Count:{}",self.tick);
        println!("Pc: {:#06x}", cpu.get_pc());
        println!("Ram value: {:#04x}", ram[cpu.get_pc() as usize]);
        println!("Name:{}",&instruc.name);
        println!("Instruction: {}", &instruc.desc);
        println!("Ticks: {}", &instruc.ticks);
        println!("");
        println!("CPU STATE____________________________________");
        println!("a:{} / {:#04x}",cpu.get_a(), cpu.get_a());
        println!("f:{} / {:#04x}",cpu.get_f(), cpu.get_f());
        println!("b:{} / {:#04x}",cpu.get_b(), cpu.get_b());
        println!("c:{} / {:#04x}",cpu.get_c(), cpu.get_c());
        println!("d:{} / {:#04x}",cpu.get_d(), cpu.get_d());
        println!("e:{} / {:#04x}",cpu.get_e(), cpu.get_e());
        println!("h:{} / {:#04x}",cpu.get_h(), cpu.get_h());
        println!("l:{} / {:#04x}",cpu.get_l(), cpu.get_l());
        println!("sp:{:#04x}",cpu.get_sp());
        println!("mie: {}",cpu.get_mie());
        println!("0xFFFF: {:#010b}",ram[0xFFFF]);
        println!("0xFF0F: {:#010b}",ram[0xFF0F]);
        println!("");
        println!("FLAGS STATE__________________________________");
        let flags = cpu.get_flags();
        println!("Z:{}",flags.Z);
        println!("N:{}",flags.N);
        println!("H:{}",flags.H);
        println!("C:{}",flags.C);
        println!("");
        println!("TIMER STATE__________________________________");
        println!("Divider:{:#04x}",ram[0xff04]);
        println!("Divider ticks:{}",timer.divider_ticks);
        println!("Timer enable:{}",timer.timer_enb);
        println!("Timer division:{}",timer.division);
        println!("Timer:{:#04x}",ram[0xff05]);
        println!("Timer ticks:{}",timer.timer_ticks);
        println!("");
        println!("INPUT STATE__________________________________");
        println!("Buttons: U:{} D:{} L:{} R:{} A:{} B:{} SE:{} ST:{}",controls.up, controls.down, controls.left, controls.right, controls.a,
        controls.b, controls.select, controls.start);
        println!("0XFF00: {:#010b}",ram[0xFF00]);
        println!("");
        println!("WARNING______________________________________");
    }

    pub fn lcd_stat(&mut self, line: u8, ram: &mut [u8;0x10000]){
        if ram[0xFF41] & 0b0100000 > 0 && line == ram[0xFF45] && self.previous_mode == H_BLANK{
            ram[0xFF0F] = ram[0xFF0F] | 0b00000010;
            println!("/!\\ STAT interrupt trigerred: LY=LYC");
            self.previous_mode = self.mode;
        }
        if ram[0xFF41] & 0b00001000 > 0 && self.mode == H_BLANK && self.mode != self.previous_mode{
            ram[0xFF0F] = ram[0xFF0F] | 0b00000010;
            println!("/!\\ STAT interrupt trigerred: H_BLANK");
            self.previous_mode = self.mode;
        }
        if ram[0xFF41] & 0b00010000 > 0 && self.mode == V_BLANK && self.mode != self.previous_mode{
            ram[0xFF0F] = ram[0xFF0F] | 0b00000010;
            println!("/!\\ STAT interrupt trigerred: V_BLANK");
            self.previous_mode = self.mode;
        }
        if ram[0xFF41] & 0b0010000 > 0 && self.mode == PX_TRANSFER && self.mode != self.previous_mode{
            ram[0xFF0F] = ram[0xFF0F] | 0b00000010;
            println!("/!\\ STAT interrupt trigerred: PX_TRANSFER");
            self.previous_mode = self.mode;
        }

    }
}

pub fn wait(){
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    print!("{esc}c", esc = 27 as char);
}

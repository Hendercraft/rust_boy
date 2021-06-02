use crate::{Hardware, Interrupts, Controls, Clock, InstrucArr};
use std::io::{stdin, stdout, Read, Write};

pub struct Master{
pub tick: u64,
pub step_by_step: bool,
pub line_by_line: bool,
pub screen_by_screen: bool,

}

impl Master{
    pub fn step(&mut self, cpu: &mut Hardware::Cpu, gpu: &mut Hardware::Gpu, ram: &mut [u8;0x10000]){
        //Check for interrupts, if none juste add 1 to PC
        if !Interrupts::interrupt_check(cpu,ram){
            cpu.set_pc(cpu.get_pc().wrapping_add(1));
        }
        let instruc : &Hardware::Instruct = cpu.fetch(ram[cpu.get_pc() as usize]);

        self.maxi_debug_print(&cpu,&gpu,&ram,&instruc);



        self.tick = self.tick.wrapping_add(instruc.ticks as u64);

        cpu.exec(ram[cpu.get_pc() as usize],ram);
        if self.step_by_step{
            wait();
        }
        cpu.set_pc(cpu.pc + cpu.fetch(ram[cpu.get_pc() as usize]).argc as u16)

    }

    pub fn screen(&mut self, cpu: &mut Hardware::Cpu, gpu: &mut Hardware::Gpu, ram: &mut [u8;0x10000]){
        for i in 0..144{
            while self.tick < 114{
                print!("{esc}c", esc = 27 as char);
                println!("SCREEN STATE__________________________________");
                println!("State: Printing");
                println!("Line: {}",i);
                println!(" ");
                self.step(cpu, gpu,ram);

            }
            self.tick = 0;
            gpu.pushLine(ram);
            if self.line_by_line {
                wait();
            }
        }
        ram[0xFFFF] += 0b00000001;
        ram[0xFF0F] += 0b00000001;
        if self.screen_by_screen {
            wait();
        }

        for i in 0..10{
            while self.tick < 114{
                print!("{esc}c", esc = 27 as char);
                println!("SCREEN STATE__________________________________");
                println!("State: V-Blank");
                println!(" ");
                self.step(cpu, gpu,ram);
            }
            self.tick = 0;
            if self.line_by_line {
                wait();
            }
        }


    }

    pub fn maxi_debug_print(&self, cpu: &Hardware::Cpu, gpu: &Hardware::Gpu, ram: &[u8;0x10000],instruc : &Hardware::Instruct){
        println!("OPERATION____________________________________");
        println!("Count:{}",self.tick);
        println!("Pc: 0x{:x}", cpu.get_pc());
        println!("Ram value: 0x{:x}", ram[cpu.get_pc() as usize]);
        println!("Name:{}",&instruc.name);
        println!("Instruction: {}", &instruc.desc);
        println!("Ticks: {}", &instruc.ticks);
        println!("");
        println!("CPU STATE____________________________________");
        println!("a:{}",cpu.get_a());
        println!("f:{}",cpu.get_f());
        println!("b:{}",cpu.get_b());
        println!("c:{}",cpu.get_c());
        println!("d:{}",cpu.get_d());
        println!("e:{}",cpu.get_e());
        println!("h:{}",cpu.get_h());
        println!("l:{}",cpu.get_l());
        println!("sp:{}",cpu.get_sp());
        println!("mie: {}",cpu.get_mie());
        println!("0xFFFF: {:b}",ram[0xFFFF]);
        println!("0xFF0F: {:b}",ram[0xFF0F]);
        println!("");
        println!("FLAGS STATE__________________________________");
        let flags = cpu.get_flags();
        println!("Z:{}",flags.Z);
        println!("N:{}",flags.N);
        println!("H:{}",flags.H);
        println!("C:{}",flags.C);
        println!("");
        println!("CONTROLS STATE_______________________________");
    }
}

pub fn wait(){
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    print!("{esc}c", esc = 27 as char);
}

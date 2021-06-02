use crate::{Hardware, Interrupts, Controls, Clock, InstrucArr};
use std::io::{stdin, stdout, Read, Write};

pub struct Master{
pub op_count: u64,
pub step_by_step: bool,
pub screen_by_screen: bool,
}

impl Master{
    pub fn step(&mut self, cpu: &mut Hardware::Cpu, gpu: &mut Hardware::Gpu, ram: &mut [u8;0x10000]){
        //Check for interrupts, if none juste add 1 to PC
        if !Interrupts::interrupt_check(cpu,ram){
            cpu.set_pc(cpu.get_pc() + 1);
        }
        let mut instruc : &Hardware::Instruct = cpu.fetch(ram[cpu.get_pc() as usize]);

        //self.maxi_debug_print(&cpu,&gpu,&ram,&instruc);

        self.op_count = self.op_count.wrapping_add(1);
        if self.step_by_step{
            let mut stdout = stdout();
            stdout.write(b"Press Enter to continue...").unwrap();
            stdout.flush().unwrap();
            stdin().read(&mut [0]).unwrap();
        }

    }

    pub fn screen(&mut self, cpu: &mut Hardware::Cpu, gpu: &mut Hardware::Gpu, ram: &mut [u8;0x10000]){
        self.step(cpu, gpu,ram);
    }

    pub fn maxi_debug_print(&self, cpu: &Hardware::Cpu, gpu: &Hardware::Gpu, ram: &[u8;0x10000],instruc : &Hardware::Instruct){
        print!("{esc}c", esc = 27 as char);
        println!("OPERATION____________________________________");
        println!("Count:{}",self.op_count * 4);
        println!("Pc: 0x{:x}", cpu.get_pc());
        println!("Ram value: 0x{:x}", ram[cpu.get_pc() as usize]);
        println!("Name:{}",&instruc.name);
        println!("Instruction: {}", &instruc.desc);
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
        println!("");
        println!("FLAGS STATE__________________________________");
        let flags = cpu.get_flags();
        println!("Z:{}",flags.Z);
        println!("N:{}",flags.N);
        println!("H:{}",flags.H);
        println!("C:{}",flags.C);
        println!("");
        println!("CONTROLS STATE_______________________________");
        println!("");
        println!("TIMER STATE__________________________________");

    }
}

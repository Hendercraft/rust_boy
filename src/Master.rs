use crate::{Hardware, Interrupts, Controls, InstrucArr, Timer};
use std::io::{stdin, stdout, Read, Write};

pub struct Master{
pub tick: u64,
pub step_by_step: bool,
pub line_by_line: bool,
pub screen_by_screen: bool,

}

impl Master{
    pub fn step(&mut self, cpu: &mut Hardware::Cpu, gpu: &mut Hardware::Gpu, timer: &mut Timer::Timer, ram: &mut [u8;0x10000]){
        //Check for interrupts, if none juste add 1 to PC
        if !Interrupts::interrupt_check(cpu,ram){
            cpu.set_pc(cpu.get_pc().wrapping_add(1));
        }
        let instruct : &Hardware::Instruct = cpu.fetch(ram[cpu.get_pc() as usize]);

        self.maxi_debug_print(&cpu,&gpu,&timer,&ram,&instruct);



        self.tick = self.tick.wrapping_add(instruct.ticks as u64);

        timer.update(instruct.ticks, ram);

        cpu.exec(instruct.n,ram);

        cpu.set_pc(cpu.pc + cpu.fetch(ram[cpu.get_pc() as usize]).argc as u16)

    }

    pub fn screen(&mut self, cpu: &mut Hardware::Cpu, gpu: &mut Hardware::Gpu, timer: &mut Timer::Timer, ram: &mut [u8;0x10000]){
        for i in 0..144{
            while self.tick < 114{
                print!("{esc}c", esc = 27 as char);
                println!("SCREEN STATE__________________________________");
                println!("State: Printing");
                println!("Line: {}",i);
                println!(" ");
                self.step(cpu, gpu, timer, ram);
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

        for j in 0..10{
            while self.tick < 114{
                print!("{esc}c", esc = 27 as char);
                println!("SCREEN STATE__________________________________");
                println!("State: V-Blank");
                println!(" ");
                self.step(cpu, gpu, timer, ram);
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


    pub fn maxi_debug_print(&self, cpu: &Hardware::Cpu, gpu: &Hardware::Gpu, timer: &Timer::Timer, ram: &[u8;0x10000],instruc : &Hardware::Instruct){
        println!("OPERATION____________________________________");
        println!("Count:{}",self.tick);
        println!("Pc: {:#06x}", cpu.get_pc());
        println!("Ram value: {:#04x}", ram[cpu.get_pc() as usize]);
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
    }
}

pub fn wait(){
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
    print!("{esc}c", esc = 27 as char);
}

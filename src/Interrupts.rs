use crate::Hardware::Cpu;

const INTERRUPTS_VBLANK: u8	 = (1 << 0);
const INTERRUPTS_LCDSTAT: u8 = (1 << 1);
const INTERRUPTS_TIMER: u8 = (1 << 2);
const INTERRUPTS_SERIAL: u8 = (1 << 3);
const INTERRUPTS_JOYPAD: u8 = (1 << 4);


fn interrupt_check(cpu : &mut Cpu, ram : &mut Vec<u8>){
    if cpu.get_mie() {
        if ram[0xFFFF as usize] & ram[0xFF0F as usize] > 0 {
            let mask: u8 = ram[0xFFFF as usize] & ram[0xFF0F as usize];

            if mask & INTERRUPTS_VBLANK > 0  {
                ram[0xFF0F as usize] &= !INTERRUPTS_VBLANK;
                vblank(cpu,ram);
            }

            if mask & INTERRUPTS_LCDSTAT > 0 {
                ram[0xFF0F as usize] &= !INTERRUPTS_LCDSTAT;
                lcd_stat(cpu,ram);
            }

            if mask & INTERRUPTS_TIMER > 0 {
                ram[0xFF0F as usize] &= !INTERRUPTS_TIMER;
                timer(cpu,ram);
            }

            if mask & INTERRUPTS_SERIAL > 0 {
                ram[0xFF0F as usize] &= !INTERRUPTS_SERIAL;
                serial(cpu,ram);
            }

            if mask & INTERRUPTS_JOYPAD > 0 {
                ram[0xFF0F as usize] &= !INTERRUPTS_JOYPAD;
                joypad(cpu,ram);
            }
        }
    }
}




pub fn vblank(cpu: &mut Cpu, ram: &mut Vec<u8>){
    cpu.set_mie(false);
    cpu.write_u16_to_stack(cpu.get_pc(),ram);
    cpu.set_pc(0x40); //+ 12 ticks
}

pub fn lcd_stat(cpu: &mut Cpu, ram: &mut Vec<u8>){
    cpu.set_mie(false);
    cpu.write_u16_to_stack(cpu.get_pc(),ram);
    cpu.set_pc(0x48); // + 12 ticks
}

pub fn timer(cpu: &mut Cpu, ram: &mut Vec<u8>){
    cpu.set_mie(false);
    cpu.write_u16_to_stack(cpu.get_pc(),ram);
    cpu.set_pc(0x50); // + 12 ticks
}

pub fn serial(cpu: &mut Cpu, ram  : &mut Vec<u8>){
    cpu.set_mie(false);
    cpu.write_u16_to_stack(cpu.get_pc(),ram);
    cpu.set_pc(0x58); // + 12 ticks
}

pub fn joypad(cpu: &mut Cpu, ram: &mut Vec<u8>){
    cpu.set_mie(false);
    cpu.write_u16_to_stack(cpu.get_pc(),ram);
    cpu.set_pc(0x60); // + 12 ticks
}

//0xD9
pub fn return_from_int (cpu: &mut Cpu, ram: & Vec<u8>){
    cpu.set_mie(true);
    let pc : u16 = cpu.read_u16_from_stack(&ram);
    cpu.set_pc(pc);
}
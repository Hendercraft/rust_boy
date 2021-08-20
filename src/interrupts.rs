use crate::hardware::Cpu;
use crate::memory::Memory;

const INTERRUPTS_VBLANK: u8 = 1 << 0;
const INTERRUPTS_LCDSTAT: u8 = 1 << 1;
const INTERRUPTS_TIMER: u8 = 1 << 2;
const INTERRUPTS_SERIAL: u8 = 1 << 3;
const INTERRUPTS_JOYPAD: u8 = 1 << 4;

pub fn interrupt_check(cpu: &mut Cpu, mem: &mut Memory) -> bool {
    if mem.read(0xFFFF) & mem.read(0xFF0F) > 0 {
        if cpu.mie {
            let mask: u8 = mem.read(0xFFFF) & mem.read(0xFF0F);

            if mask & INTERRUPTS_VBLANK > 0 {
                mem.write(0xFF0F, mem.read(0xFF0F) & !INTERRUPTS_VBLANK);
                vblank(cpu, mem);
                return true;
            }

            if mask & INTERRUPTS_LCDSTAT > 0 {
                mem.write(0xFF0F, mem.read(0xFF0F) & !INTERRUPTS_LCDSTAT);
                lcd_stat(cpu, mem);
                return true;
            }

            if mask & INTERRUPTS_TIMER > 0 {
                mem.write(0xFF0F, mem.read(0xFF0F) & !INTERRUPTS_TIMER);
                timer(cpu, mem);
                return true;
            }

            if mask & INTERRUPTS_SERIAL > 0 {
                mem.write(0xFF0F, mem.read(0xFF0F) & !INTERRUPTS_SERIAL);
                serial(cpu, mem);
                return true;
            }

            if mask & INTERRUPTS_JOYPAD > 0 {
                mem.write(0xFF0F, mem.read(0xFF0F) & !INTERRUPTS_JOYPAD);
                joypad(cpu, mem);
                return true;
            }
        }
        true
    } else {
        false
    }
}

pub fn vblank(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.mie = false;
    cpu.write_u16_to_stack(cpu.pc, mem);
    cpu.pc = 0x40; // +12 ticks
}

pub fn lcd_stat(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.mie = false;
    cpu.write_u16_to_stack(cpu.pc, mem);
    cpu.pc = 0x48; // +12 ticks
}

pub fn timer(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.mie = false;
    cpu.write_u16_to_stack(cpu.pc, mem);
    cpu.pc = 0x50; // +12 ticks
}

pub fn serial(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.mie = false;
    cpu.write_u16_to_stack(cpu.pc, mem);
    cpu.pc = 0x58; // +12 ticks
}

pub fn joypad(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.mie = false;
    cpu.write_u16_to_stack(cpu.pc, mem);
    cpu.pc = 0x60; // +12 ticks
}

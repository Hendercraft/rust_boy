use crate::hardware::Cpu;
use crate::hardware::{Flag, Flag::*};
use super::{RegU8, RegU16};

pub fn load(cpu: &mut Cpu, ram: &mut [u8; 0x10000], dest: &RegU8, src: &RegU8) {
    let value = cpu.get_reg_u8(ram, src);
    cpu.set_reg_u8(ram, dest, value);
}

pub fn load_plus(cpu: &mut Cpu, ram: &mut [u8; 0x10000], dest: &RegU8, src: &RegU8, inc: bool) {
    load(cpu, ram, dest, src);
    match inc {
        true => inc_u16(cpu, ram, &RegU16::HL),
        false => dec_u16(cpu, ram, &RegU16::HL),
    }
}

pub fn load_u16(cpu: &mut Cpu, ram: &mut [u8; 0x10000], dest: &RegU16, src: &RegU16) {
    let value = cpu.get_reg_u16(ram, src);
    cpu.set_reg_u16(ram, dest, value);
}

pub fn push(cpu: &mut Cpu, ram: &mut [u8; 0x10000], src: &RegU16) {
    let value = cpu.get_reg_u16(ram, src);
    cpu.write_u16_to_stack(value, ram);
}

pub fn pop(cpu: &mut Cpu, ram: &mut [u8; 0x10000], dest: &RegU16) {
    let value = cpu.read_u16_from_stack(ram);
    cpu.set_reg_u16(ram, dest, value);
}

pub fn add(cpu: &mut Cpu, ram: &mut [u8; 0x10000], src: &RegU8, carry: bool) {
    let lhs = cpu.get_reg_u8(ram, &RegU8::A);
    let mut rhs = cpu.get_reg_u8(ram, src);
    
    if carry & cpu.get_flag(C) {
        rhs = rhs.wrapping_add(1);
    }

    cpu.flag(C, lhs.checked_add(rhs).is_none());
    cpu.flag(H, (lhs & 0x0f) + (rhs & 0x0f) > 0x0f);

    // Compute addition
    let result = lhs.wrapping_add(rhs);
    cpu.set_reg_u8(ram, &RegU8::A, result);
    
    cpu.flag(Z, result == 0);
    cpu.flag(N, false);
}

pub fn sub(cpu: &mut Cpu, ram: &mut [u8; 0x10000], src: &RegU8, carry: bool) {
    let result = cmp(cpu, ram, src, carry);
    cpu.set_reg_u8(ram, &RegU8::A, result);
}

pub fn and(cpu: &mut Cpu, ram: &mut [u8; 0x10000], src: &RegU8) {
    cpu.flag(N, false);
    cpu.flag(H, true);
    cpu.flag(C, false);

    // Compute AND
    let result = cpu.get_reg_u8(ram, &RegU8::A) & cpu.get_reg_u8(ram, src);
    cpu.set_reg_u8(ram, &RegU8::A, result);

    cpu.flag(Z, result == 0);
}

pub fn or(cpu: &mut Cpu, ram: &mut [u8; 0x10000], src: &RegU8) {
    cpu.flag(N, false);
    cpu.flag(H, false);
    cpu.flag(C, false);

    // Compute OR
    let result = cpu.get_reg_u8(ram, &RegU8::A) | cpu.get_reg_u8(ram, src);
    cpu.set_reg_u8(ram, &RegU8::A, result);
    
    cpu.flag(Z, result == 0);
}

pub fn xor(cpu: &mut Cpu, ram: &mut [u8; 0x10000], src: &RegU8) {
    cpu.flag(N, false);
    cpu.flag(H, false);
    cpu.flag(C, false);

    // Compute OR
    let result = cpu.get_reg_u8(ram, &RegU8::A) ^ cpu.get_reg_u8(ram, src);
    cpu.set_reg_u8(ram, &RegU8::A, result);

    cpu.flag(Z, result == 0);
}

// Returns A - RegU8 (carry is used by sub())
pub fn cmp(cpu: &mut Cpu, ram: &mut [u8; 0x10000], src: &RegU8, carry: bool) -> u8 {
    let lhs = cpu.get_reg_u8(ram, &RegU8::A);
    let mut rhs = cpu.get_reg_u8(ram, src);

    if carry & cpu.get_flag(C) {
        rhs = rhs.wrapping_add(1);
    }

    cpu.flag(C, lhs < rhs);
    cpu.flag(H, lhs & 0x0f < rhs & 0x0f);

    // Compute substraction
    let result = lhs.wrapping_sub(rhs);

    cpu.flag(Z, result == 0);
    cpu.flag(N, true);
    result
}

pub fn inc(cpu: &mut Cpu, ram: &mut [u8; 0x10000], reg: &RegU8) {
    let reg_val = cpu.get_reg_u8(ram, reg);
    
    cpu.flag(H, reg_val & 0x0f == 0x0f);

    // Compute increment
    let result = reg_val.wrapping_add(1);
    cpu.set_reg_u8(ram, reg, result);

    cpu.flag(Z, result == 0);
    cpu.flag(N, false);
}

pub fn dec(cpu: &mut Cpu, ram: &mut [u8; 0x10000], reg: &RegU8) {
    let reg_val = cpu.get_reg_u8(ram, reg);
    
    // We have no bits in the lower nibble
    // We have to "borrow" some bits of the lower nibble:
    // I.e 0xf0 - 0x01 = 0xef
    cpu.flag(H, reg_val & 0x0f == 0x00);

    // Compute decrement
    let result = reg_val.wrapping_sub(1);
    cpu.set_reg_u8(ram, reg, result);

    cpu.flag(Z, result == 0);
    cpu.flag(N, true);
}

pub fn add_u16(cpu: &mut Cpu, ram: &mut [u8; 0x10000], src: &RegU16) {
    let lhs = cpu.get_reg_u16(ram, &RegU16::HL);
    let rhs = cpu.get_reg_u16(ram, src);

    cpu.flag(C, lhs.checked_add(rhs).is_none());
    cpu.flag(H, (lhs & 0x0fff) + (rhs & 0x0fff) > 0x0fff);

    cpu.set_reg_u16(ram, &RegU16::HL, lhs.wrapping_add(rhs));

    cpu.flag(N, false);
}

pub fn add_u16_i8(cpu: &mut Cpu, ram: &mut [u8; 0x10000], dest: &RegU16, offset: &RegU16) {
    let lhs = cpu.get_reg_u16(ram, dest);
    let rhs = cpu.get_reg_u16(ram, offset);

    cpu.flag(C, lhs.checked_add(rhs).is_none());
    cpu.flag(H, (lhs & 0x000f) + (rhs & 0x000f) > 0x000f);
    
    cpu.set_reg_u16(ram, dest, lhs.wrapping_add(rhs));

    cpu.flag(Z, false);
    cpu.flag(N, false);
}

pub fn inc_u16(cpu: &mut Cpu, ram: &mut [u8; 0x10000], reg: &RegU16) {
    let value = cpu.get_reg_u16(ram, reg);
    cpu.set_reg_u16(ram, reg, value.wrapping_add(1));
}

pub fn dec_u16(cpu: &mut Cpu, ram: &mut [u8; 0x10000], reg: &RegU16) {
    let value = cpu.get_reg_u16(ram, reg);
    cpu.set_reg_u16(ram, reg, value.wrapping_sub(1));
}

pub fn daa(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    let value = cpu.get_reg_u8(ram, &RegU8::A);
    let n_flag = cpu.get_flag(N);
    let mut correction = 0x00;
    
    if cpu.get_flag(C) || (!n_flag && value > 0x99) {
        correction |= 0x60;
        cpu.flag(C, true);
    }

    if cpu.get_flag(H) || (!n_flag && (value & 0x0f > 0x09)) {
        correction |= 0x06;
    }

    let result = match n_flag {
        false => value.wrapping_add(correction),
        true => value.wrapping_sub(correction),
    };
    cpu.set_reg_u8(ram, &RegU8::A, result);

    cpu.flag(Z, result == 0);
    cpu.flag(H, false);
}

pub fn cpl(cpu: &mut Cpu, ram: &mut [u8; 0x10000]) {
    cpu.flag(H, true);
    cpu.flag(N, true);

    let value = cpu.get_reg_u8(ram, &RegU8::A);
    cpu.set_reg_u8(ram, &RegU8::A, !value);
}

pub fn set_carry(cpu: &mut Cpu, _ram: &mut [u8; 0x10000], flip: bool) {
    cpu.flag(H, false);
    cpu.flag(N, false);
    cpu.flag(C, !(cpu.get_flag(C) && flip));
}

pub fn change_mie(cpu: &mut Cpu, _ram: &mut [u8; 0x10000], enable: bool) {
    cpu.pending_mie = Some(enable);
}

pub fn rotate(
    cpu: &mut Cpu, 
    ram: &mut [u8; 0x10000], 
    reg: &RegU8,
    left: bool,
    through_carry: bool, 
    update_z: bool,
    shift: bool,
    keep_msb: bool,
) {
    let value = cpu.get_reg_u8(ram, reg);
    let old_carry = match cpu.get_flag(C) {
        true => 0b1,
        false => 0b0,
    };
    
    cpu.flag(H, false);
    cpu.flag(N, false);

    let (new_carry, result) = match left {
        true => {
            let new_carry = value >> 7;
            
            let mut result = match shift {
                true => value << 1,
                false => value.rotate_left(1),
            };

            if through_carry {
                result = (result & 0b1111_1110) | old_carry;
            }

            (new_carry, result)
        },
        false => {
            let new_carry = value & 0b0000_0001;

            let mut result = match shift {
                true => value >> 1,
                false => value.rotate_right(1),
            };

            if through_carry {
                result = (result & 0b0111_1111) | (old_carry << 7);
            }

            if keep_msb {
                result = (result & 0b0111_1111) | (value & 0b1000_0000)
            }

            (new_carry, result)
        },
    };

    cpu.flag(C, new_carry != 0b0);

    cpu.set_reg_u8(ram, reg, result);

    cpu.flag(Z, update_z && result == 0);
}

pub fn swap(cpu: &mut Cpu, ram: &mut [u8; 0x10000], reg: &RegU8) {
    let value = cpu.get_reg_u8(ram, reg);
    let upper_nible = value & 0b1111_0000;
    let result = value << 4 | upper_nible >> 4;

    cpu.set_reg_u8(ram, reg, result);

    cpu.flag(N, false);
    cpu.flag(H, false);
    cpu.flag(C, false);
    cpu.flag(Z, result == 0);
}

pub fn bit(cpu: &mut Cpu, ram: &mut [u8; 0x10000], reg: &RegU8, bit_pos: u8) {
    cpu.flag(N, false);
    cpu.flag(H, true);

    let value = cpu.get_reg_u8(ram, reg);
    cpu.flag(Z, (value & 0b1 << bit_pos) == 0);
}

pub fn set_bit(cpu: &mut Cpu, ram: &mut [u8; 0x10000], reg: &RegU8, bit_pos: u8) {
    let value = cpu.get_reg_u8(ram, reg);
    let result = value | 0b1 << bit_pos;
    cpu.set_reg_u8(ram, reg, result);
}

pub fn reset_bit(cpu: &mut Cpu, ram: &mut [u8; 0x10000], reg: &RegU8, bit_pos: u8) {
    let value = cpu.get_reg_u8(ram, reg);
    let result = value & !(0b1 << bit_pos);
    cpu.set_reg_u8(ram, reg, result);
}

pub fn jump(cpu: &mut Cpu, ram: &mut [u8; 0x10000], addr: &RegU16, is_call: bool, flag: Option<&Flag>, is_set: bool) {
    let mut jump = true;
    
    if let Some(flag) = flag {
        if cpu.get_flag(*flag) != is_set {
            jump = false;
        } else {
            // Jump condition is fulfilled; add ticks
            match is_call {
                false => cpu.add_ticks(4),
                true => cpu.add_ticks(12),
            }
        }
    }

    let addr_unwrapped = cpu.get_reg_u16(ram, addr);

    if jump {
        if is_call {
            cpu.write_u16_to_stack(cpu.pc, ram);
        }

        match addr {
            // Relative jump
            RegU16::I8 => cpu.pc = cpu.pc.wrapping_add(addr_unwrapped),
            // Absolute jump
            _ => cpu.pc = addr_unwrapped,
        }
    }
}

pub fn reset(cpu: &mut Cpu, ram: &mut [u8; 0x10000], nth_byte: u16) {
    cpu.write_u16_to_stack(cpu.pc, ram);
    cpu.pc = nth_byte * 8;
}

pub fn ret(cpu: &mut Cpu, ram: &mut [u8; 0x10000], flag: Option<&Flag>, is_set: bool, i_enable: bool) {
    let mut jump = true;
    
    if let Some(flag) = flag {
        if cpu.get_flag(*flag) != is_set {
            jump = false;
        } else {
            // Jump condition is fulfilled; add ticks
            cpu.add_ticks(12);
        }
    }

    if jump {
        let addr = cpu.read_u16_from_stack(ram);
        cpu.pc = addr;

        if i_enable {
            cpu.mie = true;
        }
    }
}

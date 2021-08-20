use crate::memory::Memory;

pub struct Timer {
    pub divider_ticks: u16, //update every 256
    pub division: u16,
    pub timer_ticks: u16, //Update every division
    pub timer_enb: bool,
}

impl Timer {
    pub fn update(&mut self, ticks: u8, mem: &mut Memory) {
        self.divider_ticks += ticks as u16;

        //Up divider
        if self.divider_ticks > 255 {
            //println!("/!\\ DIVIDER OVERFLOW");
            self.divider_ticks = 0;
            // Doing this without going through the mem.write() function is essential, as it would reset to 0
            mem.ram[0x7f04] = mem.ram[0x7f04].wrapping_add(1);
        }

        //update division value
        let bits: u8 = mem.read(0xff07) & 0b00000011;
        match bits {
            0b00 => self.division = 1024,
            0b01 => self.division = 16,
            0b10 => self.division = 64,
            0b11 => self.division = 256,
            _ => {}
        }

        //update enable value
        if (mem.read(0xff07) & 0b00000100) > 0 {
            self.timer_enb = true;
        } else {
            self.timer_enb = false;
        }

        //update timer
        if self.timer_enb {
            self.timer_ticks += ticks as u16;
            if self.timer_ticks > self.division {
                if mem.read(0xff05) == 255 {
                    //println!("/!\\ TIMER OVERFLOW");
                    mem.write(0xff05, mem.read(0xff06));
                    mem.write(0xff0f, mem.read(0xff0f) | 0b00000100);
                } else {
                    mem.write(0xff05, mem.read(0xff05) + 1);
                }
                self.timer_ticks = 0;
            }
        }
    }
}

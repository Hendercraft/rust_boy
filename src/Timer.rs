pub struct Timer{
    pub divider_ticks : u16,//update every 256
    pub division : u16,
    pub timer_ticks : u16, //Update every division
    pub timer_enb : bool

}

impl Timer{
    pub fn update(&mut self, ticks:u8, ram: &mut [u8;0x10000]){

        self.divider_ticks += ticks as u16;


        //Up divider
        if self.divider_ticks > 255 {
            self.divider_ticks = 0;
            ram[0xff04] = ram[0xff04].wrapping_add(1);
        }

        //update division value
        let bits:u8 = ram[0xff07] & 0b00000011;
        match bits{
            0b00 => self.division = 1024,
            0b01 => self.division = 16,
            0b10 => self.division = 64,
            0b11 => self.division = 256,
            _ => {}
        }

        //update enable value
        if (ram[0xff07] & 0b00000100) > 0{
            self.timer_enb = true;
        }else{
            self.timer_enb = false;
        }

        //update timer
        if self.timer_enb{
            self.timer_ticks += ticks as u16 ;
            if self.timer_ticks > self.division{
                if ram[0xff05] == 255{
                    ram[0xff05] = ram[0xff06];
                    ram[0xff0f] = ram[0xff0f] | 0b00000100;
                }else{
                    ram[0xff05] += 1;
                }
                self.timer_ticks = 0;
            }
        }
    }
}

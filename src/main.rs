fn main(){}
mod hardware {
    
    struct Flags{
        Z : bool,
        N : bool,
        H : bool,
        C : bool,
    }
    
    struct Cpu{ 
        a : u8,
        f : u8,
        b : u8,
        c : u8,
        d : u8,
        e : u8,
        h : u8,
        l : u8,
        sp : u16,
        pc : u16,
        //flags : Flags,
        instruc : [Instruc;256],
    }
    
    struct Gpu{
        screen : [[u8;160];144], 
        matrix : [[u8;160];144],    
    }    


    struct Instruc {
        n : u16,
        name : String,
        desc : String,
        argc : u8,
        tics : u8,
        exec : fn(&mut Cpu),
    }
    

    impl Cpu {
        
        fn get_u16 (h: u8 ,l: u8) -> u16{return 8;}

        fn set_u16 (h: u8,l: u8,n: u16){}

        fn get_flags (&self) -> Flags{
            let output : Flags;
            let temp = self.f >> 4;         
            output.Z = temp & 0b1000 > 0; //get upper
            output.N = temp & 0b0100 > 0;
            output.H = temp & 0b0010 > 0;
            output.C = temp & 0b0001 > 0;
            return output;
        }

        fn set_flags (f: u8){}
        
        fn fetch<'a>(&self,i: u8) -> Option<&'a Instruc> {return self.instruc[i];}

        fn exec<'a>(i :&Instruc){}


    }
}

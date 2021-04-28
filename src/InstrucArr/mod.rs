use crate::Hardware;
mod InstrucFn;

fn createOperations() -> Vec<Hardware::Instruc>{
    let mut out = Vec::new();
    for i in 0..255{
        out.push(Hardware::Instruc{
            n : i,
            name : String::from("NOP"),
            desc : String::from("Aussi inutile que les cours de GE00"),
            argc : 1,
            tics : 4,
            exec : InstrucFn::nop,
        })
    }
    return out;
}

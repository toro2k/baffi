use std::io::Read;
use std::io::Result;

use vm::Inst;


const INC: u8 = 43; // +
const INPUT: u8 = 44; // ,
const DEC: u8 = 45; // -
const OUTPUT: u8 = 46; // .
const PREV: u8 = 60; // <
const NEXT: u8 = 62; // >
// const LOOP_BEG: u8 = 91; // [
// const LOOP_END: u8 = 93; // ]


pub fn read_and_strip_bf_code<T: Read>(input: T) -> Result<Vec<Inst>> {
    let compiler = Compiler::new(input);
    compiler.compile()
}

pub struct Compiler<In: Read> {
    input: In,
}

impl<In: Read> Compiler<In> {
    pub fn new(input: In) -> Compiler<In> {
        Compiler { input: input }
    }

    pub fn compile(self) -> Result<Vec<Inst>> {
        let mut code = vec![];
        for maybe_byte in self.input.bytes() {
            let byte = try!(maybe_byte);
            match byte {
                INC => code.push(Inst::Inc),
                DEC => code.push(Inst::Dec),
                NEXT => code.push(Inst::Next),
                PREV => code.push(Inst::Prev),
                INPUT => code.push(Inst::Input),
                OUTPUT => code.push(Inst::Output),
                _ => continue,
            }
        }
        Ok(code)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use vm::Inst::{Inc};

    #[test]
    fn foo() {
        let compiler = Compiler::new("+".as_bytes());
        let code = compiler.compile().unwrap();
        assert_eq!(vec![Inc], code);
    }
}

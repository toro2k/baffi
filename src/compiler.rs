use std::io::Read;
use std::io::Result;

use vm::Inst;


const INC: u8 = 43; // +
const INPUT: u8 = 44; // ,
const DEC: u8 = 45; // -
const OUTPUT: u8 = 46; // .
const PREV: u8 = 60; // <
const NEXT: u8 = 62; // >
const LOOP_BEG: u8 = 91; // [
const LOOP_END: u8 = 93; // ]


pub fn compile_bf<T: Read>(input: T) -> Result<Vec<Inst>> {
    let mut code = vec![];
    let mut counter = 0;
    let mut loop_stack = vec![];

    for maybe_byte in input.bytes() {
        let byte = try!(maybe_byte);
        match byte {
            INC => code.push(Inst::Inc),
            DEC => code.push(Inst::Dec),
            NEXT => code.push(Inst::Next),
            PREV => code.push(Inst::Prev),
            INPUT => code.push(Inst::Input),
            OUTPUT => code.push(Inst::Output),

            LOOP_BEG => {
                loop_stack.push(counter);
                code.push(Inst::JumpIfZero(0));
            },

            LOOP_END => {
                // TODO: loop_stack may be empty -> unmatched brackets
                let matching_bracket_counter = loop_stack.pop().unwrap();
                code.push(Inst::JumpUnlessZero(matching_bracket_counter + 1));
                code[matching_bracket_counter] = Inst::JumpIfZero(counter + 1);
            },

            _ => {
                // FIXME: here crashes if the program start with a non command byte
                counter -= 1;
            },
        }
        counter += 1;
    }
    Ok(code)
}

#[cfg(test)]
mod test {

    use super::*;
    use vm::Inst::*;

    #[test]
    fn compile_simple_instructions() {
        let code = compile_bf("+-><,.".as_bytes()).unwrap();
        assert_eq!(vec![Inc, Dec, Next, Prev, Input, Output], code);
    }

    #[test]
    fn compile_empty_loop() {
        let code = compile_bf("[]".as_bytes()).unwrap();
        assert_eq!(vec![JumpIfZero(2), JumpUnlessZero(1)], code);
    }

    #[test]
    fn foo() {
        let code = compile_bf("+[-]".as_bytes()).unwrap();
        assert_eq!(vec![Inc, JumpIfZero(4), Dec, JumpUnlessZero(2)], code);
    }
}

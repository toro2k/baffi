use std::io::Read;
use std::io::Result;

use vm::Inst;


const PLUS: u8 = 43; // +
const COMMA: u8 = 44; // ,
const MINUS: u8 = 45; // -
const DOT: u8 = 46; // .
const LT: u8 = 60; // <
const GT: u8 = 62; // >
const LBRACK: u8 = 91; // [
const RBRACK: u8 = 93; // ]


pub fn compile_bf<T: Read>(input: T) -> Result<Vec<Inst>> {
    let mut code = vec![];
    let mut counter = 0;
    let mut loop_stack = vec![];

    for maybe_byte in input.bytes() {
        let byte = try!(maybe_byte);

        if !is_brainfuck_byte(byte) {
            continue;
        }

        match byte {
            PLUS => code.push(Inst::Inc),
            MINUS => code.push(Inst::Dec),
            GT => code.push(Inst::Next),
            LT => code.push(Inst::Prev),
            COMMA => code.push(Inst::Input),
            DOT => code.push(Inst::Output),

            LBRACK => {
                loop_stack.push(counter);
                code.push(Inst::JumpIfZero(0));
            },

            RBRACK => {
                // TODO: loop_stack may be empty -> unmatched brackets
                let matching_bracket_counter = loop_stack.pop().unwrap();
                code.push(Inst::JumpUnlessZero(matching_bracket_counter + 1));
                code[matching_bracket_counter] = Inst::JumpIfZero(counter + 1);
            },

            _ => panic!("BUG!"),

        }

        counter += 1;
    }
    Ok(code)
}

fn is_brainfuck_byte(byte: u8) -> bool {
    byte == PLUS || byte == MINUS ||
    byte == GT || byte == LT ||
    byte == DOT || byte == COMMA ||
    byte == LBRACK || byte == RBRACK
}


#[cfg(test)]
mod test {

    use super::*;
    use vm::Inst::*;

    #[test]
    fn initial_non_command_characters_doesnt_panic_the_compiler() {
        // checks I don't do stupid things with the instructions counter!
        let code = compile_bf("a+".as_bytes()).unwrap();
        assert_eq!(vec![Inc], code);
    }

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

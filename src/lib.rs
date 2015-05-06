use std::io::Read;
use std::io::Error;


pub const INC: u8 = 43; // +
pub const GET: u8 = 44; // ,
pub const DEC: u8 = 45; // -
pub const PUT: u8 = 46; // .
pub const PREV: u8 = 60; // <
pub const NEXT: u8 = 62; // >
pub const LOOP_BEG: u8 = 91; // [
pub const LOOP_END: u8 = 93; // ]


#[derive(Debug, PartialEq)]
pub struct Tape {
    raw_tape: Vec<i8>,
    pointer: usize,
}

impl Tape {
    pub fn new(size: usize) -> Option<Tape> {
        if size > 0 {
            Some(Tape { raw_tape: vec![0; size], pointer: 0 })
        } else {
            None
        }
    }

    pub fn inc_cell(&mut self) {
        let v = self.get_cell();
        self.put_cell(v.wrapping_add(1));
    }

    pub fn dec_cell(&mut self) {
        let v = self.get_cell();
        self.put_cell(v.wrapping_sub(1));
    }

    pub fn get_cell(&self) -> i8 {
        self.raw_tape[self.pointer]
    }

    pub fn put_cell(&mut self, value: i8) {
        self.raw_tape[self.pointer] = value;
    }
}


pub fn read_and_strip_bf_code<T: Read>(input: T) -> Result<Vec<u8>, Error> {
    let mut code = vec![];
    for maybe_byte in input.bytes() {
        let byte = try!(maybe_byte);
        if is_bf_cmd(byte) {
            code.push(byte);
        } else {
            continue;
        }
    }
    Ok(code)
}

pub fn eval_bf(code: &[u8], tape: &mut Tape) {
    let mut pc = 0;

    while pc < code.len() {
        let cmd = code[pc];

        match cmd {
            INC => tape.inc_cell(),
            DEC => tape.dec_cell(),
            _ => { /* not implemented */ },
        }

        pc += 1;
    }
}

fn is_bf_cmd(byte: u8) -> bool {
    byte == INC || byte == DEC ||
    byte == GET || byte == PUT ||
    byte == NEXT || byte == PREV ||
    byte == LOOP_END || byte == LOOP_BEG
}


#[cfg(test)]
mod test {

    use super::*;

    // is this the proper way?
    // see libcollections/macros.rs
    macro_rules! tape {
        ( $( $x:expr ),+ ) => (
            Tape { raw_tape: vec![$($x),+], pointer: 0, }
        );
    }

    #[test]
    fn i_can_use_a_string_as_input() {
        let code = read_and_strip_bf_code("+".as_bytes());
        assert_eq!(vec![INC], code.unwrap());
    }

    #[test]
    fn test_tape_macro() {
        let expected = Tape { raw_tape: vec![0], pointer: 0, };
        assert_eq!(expected, tape![0]);
    }

    #[test]
    fn test_inc_and_dec() {
        let mut tape = tape![0];
        eval_bf("++-".as_bytes(), &mut tape);
        assert_eq!(tape![1], tape);
    }
}

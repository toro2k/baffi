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


#[derive(Debug)]
pub struct Vm {
    memory: Vec<i8>,
    pointer: usize,
}

impl Vm {
    pub fn new(size: usize) -> Option<Vm> {
        if size > 0 {
            Some(Vm { memory: vec![0; size], pointer: 0 })
        } else {
            None
        }
    }

    pub fn eval(&mut self, code: &[u8]) {
        let mut pc = 0;

        while pc < code.len() {
            let cmd = code[pc];

            match cmd {
                INC => self.inc_cell(),
                DEC => self.dec_cell(),
                _ => { /* not implemented */ },
            }

            pc += 1;
        }
    }

    fn inc_cell(&mut self) {
        let v = self.get_cell();
        self.put_cell(v.wrapping_add(1));
    }

    fn dec_cell(&mut self) {
        let v = self.get_cell();
        self.put_cell(v.wrapping_sub(1));
    }

    fn get_cell(&self) -> i8 {
        self.memory[self.pointer]
    }

    fn put_cell(&mut self, value: i8) {
        self.memory[self.pointer] = value;
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

fn is_bf_cmd(byte: u8) -> bool {
    byte == INC || byte == DEC ||
    byte == GET || byte == PUT ||
    byte == NEXT || byte == PREV ||
    byte == LOOP_END || byte == LOOP_BEG
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn i_can_use_a_string_as_input() {
        let code = read_and_strip_bf_code("+".as_bytes());
        assert_eq!(vec![INC], code.unwrap());
    }

    #[test]
    fn test_inc_and_dec() {
        let mut vm = Vm::new(1).unwrap();
        vm.eval("++-".as_bytes());
        assert_eq!(vec![1], vm.memory);
    }

    #[test]
    fn integer_arithmetic_wraps_around() {
        let mut vm = Vm::new(1).unwrap();
        let mut code = String::new();
        for _ in 0..256 { code.push('+'); }
        vm.eval(code.as_bytes());
        assert_eq!(0, vm.memory[0]);
    }
}

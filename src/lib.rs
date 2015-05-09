use std::io::Read;
use std::io::Write;
use std::io::Result;


const INC: u8 = 43; // +
const INPUT: u8 = 44; // ,
const DEC: u8 = 45; // -
const OUTPUT: u8 = 46; // .
const PREV: u8 = 60; // <
const NEXT: u8 = 62; // >
// const LOOP_BEG: u8 = 91; // [
// const LOOP_END: u8 = 93; // ]


#[derive(Debug, PartialEq)]
pub enum Inst {
    Inc,
    Dec,
    Next,
    Prev,
    Input,
    Output,
}

#[derive(Debug)]
pub struct Vm<In, Out> {
    memory: Vec<i8>,
    pointer: usize,
    input: In,
    output: Out,
}

impl<In: Read, Out: Write> Vm<In, Out> {
    pub fn new(size: usize, input: In, output: Out) -> Option<Vm<In, Out>> {
        if size > 0 {
            Some(Vm { memory: vec![0; size], pointer: 0, input: input, output: output })
        } else {
            None
        }
    }

    pub fn eval(&mut self, code: &[Inst]) {
        let mut pc = 0;

        while pc < code.len() {
            let cmd = &code[pc];

            match cmd {
                &Inst::Inc => self.inc_cell(),
                &Inst::Dec => self.dec_cell(),
                &Inst::Next => self.next_cell(),
                &Inst::Prev => self.prev_cell(),
                &Inst::Input => self.read_cell(),
                &Inst::Output => self.write_cell(),
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

    fn next_cell(&mut self) {
        // TODO: need a better condition
        if self.pointer < (self.memory.len() - 1) {
            self.pointer += 1;
        }
    }

    fn prev_cell(&mut self) {
        if self.pointer > 0 {
            self.pointer -= 1;
        }
    }

    fn read_cell(&mut self) {
        // TODO: there is no better/simpler way?
        // TODO: where the literal `[0]` is allocated here?
        let i: &mut [u8] = &mut [0];
        self.input.read(&mut i[0..1]).unwrap();
        self.put_cell(i[0] as i8);
    }

    fn write_cell(&mut self) {
        let o: &[u8] = &[self.get_cell() as u8];
        self.output.write(o).unwrap();
    }

    fn get_cell(&self) -> i8 {
        self.memory[self.pointer]
    }

    fn put_cell(&mut self, value: i8) {
        self.memory[self.pointer] = value;
    }
}

pub fn read_and_strip_bf_code<T: Read>(input: T) -> Result<Vec<Inst>> {
    let mut code = vec![];
    for maybe_byte in input.bytes() {
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


#[cfg(test)]
mod test {

    use super::*;
    use super::Inst::{Inc,Dec,Next,Prev,Input,Output};

    // TODO: define assert_vm_memory_eq!
    // TODO: define make_vm(usize) -> Vm<?!?>: empty input from string
    // TODO: define make_vm_with_input<In: Read>(usize, In) -> Vm<In>

    #[test]
    fn i_can_use_a_string_as_input() {
        let code = read_and_strip_bf_code("+".as_bytes());
        assert_eq!(vec![Inc], code.unwrap());
    }

    #[test]
    fn test_inc_and_dec() {
        let mut output = Vec::new();
        let mut vm = Vm::new(1, "".as_bytes(), &mut output).unwrap();

        vm.eval(&[Inc, Inc, Dec]);
        assert_eq!(vec![1], vm.memory);
    }

    #[test]
    fn test_next_and_prev() {
        let mut output = Vec::new();
        let mut vm = Vm::new(2, "".as_bytes(), &mut output).unwrap();

        vm.eval(&[Next, Inc, Prev, Inc]);
        assert_eq!(vec![1, 1], vm.memory);
    }

    #[test]
    fn cant_move_beyond_end_of_memory() {
        let mut output = Vec::new();
        let mut vm = Vm::new(1, "".as_bytes(), &mut output).unwrap();
        let code = &[Next, Inc];

        vm.eval(code);
        assert_eq!(vec![1], vm.memory);
    }

    #[test]
    fn test_input() {
        let mut output = Vec::new();
        let mut vm = Vm::new(2, "\u{01}\u{10}".as_bytes(), &mut output).unwrap();

        vm.eval(&[Input, Next, Input]);
        assert_eq!(vec![0x1, 0x10], vm.memory);
    }

    #[test]
    fn read_end_of_input_set_cell_to_zero() {
        let mut output = Vec::new();
        let mut vm = Vm::new(1, "\u{01}".as_bytes(), &mut output).unwrap();
        let code = &[Input, Input];

        vm.eval(code);
        assert_eq!(vec![0], vm.memory);
    }

    #[test]
    fn test_output() {
        let mut output = Vec::new();
        {
            let mut vm = Vm::new(1, "".as_bytes(), &mut output).unwrap();
            let code = &[Output, Inc, Output];
            vm.eval(code);
        }
        assert_eq!(vec![0, 1], output);
    }

    #[test]
    fn integer_arithmetic_wraps_around() {
        let mut code = vec![];
        for _ in 0..256 {
            code.push(Inc);
        }
        let mut output = Vec::new();
        let mut vm = Vm::new(1, "".as_bytes(), &mut output).unwrap();

        vm.eval(&code);
        assert_eq!(0, vm.memory[0]);
    }
}

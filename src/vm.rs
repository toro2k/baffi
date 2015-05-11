use std::io::Read;
use std::io::Write;


#[derive(Debug, PartialEq)]
pub enum Inst {
    Inc,
    Dec,
    Next,
    Prev,
    Input,
    Output,
    JumpIfZero(usize),
    JumpUnlessZero(usize),
}

#[derive(Debug)]
pub struct Vm<In, Out> {
    memory: Vec<u8>,
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
                &Inst::JumpIfZero(addr) => {
                    if self.get_cell() == 0 {
                        pc = addr;
                        continue;
                    }
                }
                &Inst::JumpUnlessZero(addr) => {
                    if self.get_cell() != 0 {
                        pc = addr;
                        continue;
                    }
                }
            }

            pc += 1;
        }
    }

    fn inc_cell(&mut self) {
        self.memory[self.pointer] += 1;
    }

    fn dec_cell(&mut self) {
        self.memory[self.pointer] -= 1;
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
        self.put_cell(i[0]);
    }

    fn write_cell(&mut self) {
        if self.get_cell() != 0 {
            self.output.write(&self.memory[self.pointer..1]).unwrap();
        }
    }

    fn get_cell(&self) -> u8 {
        self.memory[self.pointer]
    }

    fn put_cell(&mut self, value: u8) {
        self.memory[self.pointer] = value;
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use super::Inst::*;

    // TODO: define assert_vm_memory_eq!
    // TODO: define make_vm(usize) -> Vm<?!?>: empty input from string
    // TODO: define make_vm_with_input<In: Read>(usize, In) -> Vm<In>

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
            let code = &[Inc, Output, Inc, Output];
            vm.eval(code);
        }
        assert_eq!(vec![1, 2], output);
    }

    #[test]
    fn null_bytes_arent_sent_to_output() {
        let mut output = Vec::new();
        {
            let mut vm = Vm::new(1, "".as_bytes(), &mut output).unwrap();
            let code = &[Output, Inc, Output];
            vm.eval(code);
        }
        assert_eq!(vec![1], output);
    }

    #[test]
    fn an_empty_loop_doesnt_do_anything() {
        // See compiler::test::compile_empty_loop()
        let mut output = Vec::new();
        let mut vm = Vm::new(1, "".as_bytes(), &mut output).unwrap();
        let code = &[JumpIfZero(2), JumpUnlessZero(1)];

        vm.eval(code);
        assert_eq!(vec![0], vm.memory);
    }

    #[test]
    fn loop_is_working_properly() {
        let mut output = Vec::new();
        let mut vm = Vm::new(1, "".as_bytes(), &mut output).unwrap();
        let code = &[Inc, JumpIfZero(4), Dec, JumpUnlessZero(2)];

        vm.eval(code);
        assert_eq!(vec![0], vm.memory);
    }
}

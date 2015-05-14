use std::fmt;
use std::io;
use std::io::Read;
use std::io::Write;
use std::result;


pub struct Vm<In, Out> {
    memory: Vec<u8>,
    pointer: usize,
    input: In,
    output: Out,
}

impl<In: Read, Out: Write> Vm<In, Out> {
    pub fn new(size: usize, input: In, output: Out) -> Vm<In, Out> {
        if size > 0 {
            Vm {
                memory: vec![0; size],
                pointer: 0,
                input: input,
                output: output
            }
        } else {
            panic!("memory cannot be empty");
        }
    }

    pub fn eval(&mut self, code: &[Inst]) -> Result {
        let mut counter = 0;

        while counter < code.len() {
            let cmd = &code[counter];

            match *cmd {
                Inst::Inc => self.inc_cell(),
                Inst::Dec => self.dec_cell(),

                Inst::Next => try!(self.next_cell()),
                Inst::Prev => try!(self.prev_cell()),

                Inst::Input => try!(self.read_cell()),
                Inst::Output => try!(self.write_cell()),

                Inst::JumpIfZero(addr) => {
                    if self.curr_cell_value() == 0 {
                        counter = addr;
                        continue;
                    }
                }
                Inst::JumpUnlessZero(addr) => {
                    if self.curr_cell_value() != 0 {
                        counter = addr;
                        continue;
                    }
                }
            }

            counter += 1;
        }

        Ok(())
    }

    pub fn into_inner(self) -> (In, Out) {
        (self.input, self.output)
    }

    fn inc_cell(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
    }

    fn dec_cell(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
    }

    fn next_cell(&mut self) -> Result {
        if self.pointer < self.memory.len()-1 {
            self.pointer += 1;
            Ok(())
        } else {
            Err(RuntimeError::pointer_out_of_bounds())
        }
    }

    fn prev_cell(&mut self) -> Result {
        if self.pointer > 0 {
            self.pointer -= 1;
            Ok(())
        } else {
            Err(RuntimeError::pointer_out_of_bounds())
        }
    }

    fn read_cell(&mut self) -> Result {
        let pointer_as_range = self.pointer..self.pointer+1;
        match self.input.read(&mut self.memory[pointer_as_range]) {
            Ok(_) => Ok(()),
            Err(_) => Err(RuntimeError::io_error()),
        }
    }

    fn write_cell(&mut self) -> Result {
        let pointer_as_range = self.pointer..self.pointer+1;
        match self.output.write(&self.memory[pointer_as_range]) {
            Ok(_) => Ok(()),
            Err(_) => Err(RuntimeError::io_error()),
        }
    }

    fn curr_cell_value(&self) -> u8 {
        self.memory[self.pointer]
    }
}

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

pub type Result = result::Result<(), RuntimeError>;

#[derive(Debug, PartialEq)]
pub struct RuntimeError {
    description: &'static str,
}

impl RuntimeError {
    fn pointer_out_of_bounds() -> RuntimeError {
        RuntimeError { description: "pointer out of bounds" }
    }

    fn io_error() -> RuntimeError {
        RuntimeError { description: "io error" }
    }
}

impl From<io::Error> for RuntimeError {
    fn from(_: io::Error) -> RuntimeError {
        RuntimeError::io_error()
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description)
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use super::Inst::*;

    #[test]
    fn test_inc_and_dec() {
        let mut vm = Vm::new(1, "".as_bytes(), vec![]);

        vm.eval(&[Inc, Inc, Dec]).unwrap();
        assert_eq!(vec![1], vm.memory);
    }

    #[test]
    fn inc_wraps_around() {
        let mut vm = Vm::new(1, "".as_bytes(), vec![]);
        let mut code = vec![];
        for _ in 0..256 { code.push(Inc); }

        vm.eval(&code).unwrap();
        assert_eq!(vec![0], vm.memory);
    }

    #[test]
    fn dec_wraps_around() {
        let mut vm = Vm::new(1, "".as_bytes(), vec![]);
        let mut code = vec![];
        for _ in 0..256 { code.push(Dec); }

        vm.eval(&code).unwrap();
        assert_eq!(vec![0], vm.memory);
    }

    #[test]
    fn test_next_and_prev() {
        let mut vm = Vm::new(2, "".as_bytes(), vec![]);

        vm.eval(&[Next, Inc, Prev, Inc]).unwrap();
        assert_eq!(vec![1, 1], vm.memory);
    }

    #[test]
    fn move_beyond_end_of_memory_is_an_error() {
        let mut vm = Vm::new(1, "".as_bytes(), vec![]);
        let code = &[Next, Inc];

        let error = vm.eval(code).unwrap_err();
        assert_eq!(RuntimeError::pointer_out_of_bounds(), error);
    }

    #[test]
    fn move_below_begin_of_memory_is_an_error() {
        let mut vm = Vm::new(1, "".as_bytes(), vec![]);
        let code = &[Prev];

        let error = vm.eval(code).unwrap_err();
        assert_eq!(RuntimeError::pointer_out_of_bounds(), error);
    }

    #[test]
    fn test_input() {
        let mut vm = Vm::new(2, "\u{01}\u{10}".as_bytes(), vec![]);

        vm.eval(&[Input, Next, Input]).unwrap();
        assert_eq!(vec![0x1, 0x10], vm.memory);
    }

    #[test]
    fn read_end_of_input_leave_pointed_cell_as_is() {
        let mut vm = Vm::new(1, "".as_bytes(), vec![]);
        let code = &[Inc, Input];

        vm.eval(code).unwrap();
        assert_eq!(vec![1], vm.memory);
    }

    #[test]
    fn test_output() {
        let mut vm = Vm::new(1, "".as_bytes(), vec![]);
        let code = &[Inc, Output, Inc, Output];
        vm.eval(code).unwrap();

        let (_, output) = vm.into_inner();
        assert_eq!(vec![1, 2], output);
    }

    #[test]
    fn an_empty_loop_doesnt_do_anything() {
        let mut vm = Vm::new(1, "".as_bytes(), vec![]);
        let code = &[JumpIfZero(2), JumpUnlessZero(1)];

        vm.eval(code).unwrap();
        assert_eq!(vec![0], vm.memory);
    }

    #[test]
    fn clear_loop_sets_a_cell_to_zero() {
        let mut vm = Vm::new(1, "".as_bytes(), vec![]);
        let code = &[Inc, JumpIfZero(4), Dec, JumpUnlessZero(2)];

        vm.eval(code).unwrap();
        assert_eq!(vec![0], vm.memory);
    }
}

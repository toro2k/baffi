extern crate bf;

use std::io;
use std::env;
use std::fs::File;


pub fn main() {
    if let Some(ref file_rel_path) = env::args().nth(1) {
        match File::open(file_rel_path) {
            Ok(input) => eval_from_input(input),
            Err(why) => println!("cannot open file: {}", why),
        }
    } else {
        println!("missing file path argument");
    }
}

fn eval_from_input(input: File) {
    match bf::compiler::compile_bf(input) {
        Ok(code) => {
            let mut vm = bf::vm::Vm::new(MEMORY_SIZE, io::stdin(), io::stdout()).unwrap();
            vm.eval(&code);
        },

        Err(error) => println!("{}", error),
    }
}


const MEMORY_SIZE: usize = 30000;

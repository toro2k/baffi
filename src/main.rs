extern crate bf;

use std::io;
use std::env;
use std::fs;

use bf::compiler;
use bf::eval;


pub fn main() {
    if let Some(ref file_rel_path) = env::args().nth(1) {
        match fs::File::open(file_rel_path) {
            Ok(input) => eval_from_input(input),
            Err(why) => println!("cannot open file: {}", why),
        }
    } else {
        println!("missing file path argument");
    }
}

fn eval_from_input(input: fs::File) {
    match compiler::compile_bf(input) {
        Ok(code) => {
            let input = io::stdin();
            let output = io::stdout();
            let mut vm = eval::Vm::new(MEMORY_SIZE, input, output);
            if let Err(why) = vm.eval(&code) {
                println!("runtime error: {}", why);
            }
        },

        Err(why) => println!("compiler error: {}", why),
    }
}


const MEMORY_SIZE: usize = 65536;

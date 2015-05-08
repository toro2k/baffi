extern crate bf;

use std::io;
use std::env;
use std::fs::File;

// why pub?
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
    match bf::read_and_strip_bf_code(input) {
        Ok(code) => {
            let mut vm = bf::Vm::new(10, io::stdin()).unwrap();
            vm.eval(&code);
        },

        Err(why) => println!("error reading input: {}", why),
    }
}

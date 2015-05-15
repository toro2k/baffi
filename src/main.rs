extern crate baffi;

use std::io;
use std::io::Write;
use std::env;
use std::fs;

use baffi::compiler;
use baffi::eval;


macro_rules! printerrln {
    ($fmt:expr) => ((
        writeln!(&mut io::stderr(), $fmt).unwrap()
    ));

    ($fmt:expr, $($args:tt)*) => ((
        writeln!(&mut io::stderr(), $fmt, $($args)*).unwrap()
    ));
}


pub fn main() {
    if let Some(ref file_rel_path) = env::args().nth(1) {
        match fs::File::open(file_rel_path) {
            Ok(input) => eval_from_input(input),
            Err(why) => printerrln!("cannot open file: {}", why),
        }
    } else {
        printerrln!("missing file path argument");
    }
}

fn eval_from_input(input: fs::File) {
    match compiler::compile_bf(input) {
        Ok(code) => {
            let input = io::stdin();
            let output = io::stdout();
            let mut vm = eval::Vm::new(MEMORY_SIZE, input, output);
            if let Err(why) = vm.eval(&code) {
                printerrln!("runtime error: {}", why);
            }
        },

        Err(why) => printerrln!("compiler error: {}", why),
    }
}


const MEMORY_SIZE: usize = 65536;

extern crate bf;

use std::fs::File;

// why pub?
pub fn main() {
    if let Some(ref file_rel_path) = std::env::args().nth(1) {
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
            if let Some(mut tape) = bf::Tape::new(10) {
                bf::eval_bf(&code, &mut tape);
                println!("{:?}", tape);
            } else {
                println!("cannot eval on an empty tape");
            }
        },

        Err(why) => println!("error reading input: {}", why),
    }
}

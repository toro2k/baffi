use std::io;
use std::io::Read;

const INC: u8 = 43; // +
const GET: u8 = 44; // ,
const DEC: u8 = 45; // -
const PUT: u8 = 46; // .
const PREV: u8 = 60; // <
const NEXT: u8 = 62; // >
const LOOP_BEG: u8 = 123; // {
const LOOP_END: u8 = 125; // }


fn main() {
    let code = compile_bf();
    eval_bf(&code);
}

#[derive(Debug)]
enum Cmd {
    Simple(u8) , Loop(Vec<Cmd>)
}

fn compile_bf() -> Vec<Cmd> {
    let mut code = vec![];
    let reader = io::stdin();

    // select + map? how to handle loops?
    for byte in reader.bytes() {
        match byte {
            Ok(INC) => code.push(Cmd::Simple(INC)),
            Ok(DEC) => code.push(Cmd::Simple(DEC)),
            Ok(NEXT) => code.push(Cmd::Simple(NEXT)),
            Ok(PREV) => code.push(Cmd::Simple(PREV)),
            Ok(GET) => code.push(Cmd::Simple(GET)),
            Ok(PUT) => code.push(Cmd::Simple(PUT)),
            Ok(LOOP_BEG) => code.push(Cmd::Loop(vec![])),
            Ok(LOOP_END) => { /* close loop */ },
            Ok(_) => continue,
            Err(why) => panic!("cannot read input: {}", why),
        }
    }
    code
}

fn eval_bf(code: &[Cmd]) {
    println!("{:?}", code);
}

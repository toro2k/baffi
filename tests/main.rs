extern crate bf;

use std::io;

#[test]
fn simple_program() {
    assert!(true);
    let mut output = Vec::new();
    {
        let mut vm = bf::vm::Vm::new(9999, io::stdin(), &mut output).unwrap();
        let code = bf::compiler::compile_bf("+.".as_bytes()).unwrap();
        vm.eval(&code);
    }
    assert_eq!(vec![1], output);
}

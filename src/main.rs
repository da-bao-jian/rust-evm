mod evm;
use evm::{
    opcode,
    vm
};
use std::{
    fs,
    error::Error,
    string::ParseError,
    env
};

fn debug(machine: &vm::VM) {

}

fn execute(machine: &vm::VM) {

}


#[allow(unused_variables)]
fn main() {

    let args: Vec<String> = env::args().collect();
    let action = args[0].clone();
    let filename = args[1].clone();

    // loading the vm 
    let mut vm = vm::VM::new(&filename).unwrap();

    match &action[..] {
        "debug" => debug(&mut vm),
        "execute" => execute(&mut vm),
        _ => return
    }

}

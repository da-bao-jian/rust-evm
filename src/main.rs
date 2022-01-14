mod evm;
use evm::{
    opcode::{self, OpCode},
    vm
};
use std::{
    fs,
    error::Error,
    string::ParseError,
    env
};

fn execute(machine: &mut vm::VM, debug: bool) {
    loop {
        match machine.next() {
            Some(x) => if !debug {
                x.step()
            }else {
                x.describe()
            }
            _ => panic!("Reached end of the line"),
            None => panic!("Opcode non-recognizable"),
        }
    }
}


#[allow(unused_variables)]
fn main() {

    let args: Vec<String> = env::args().collect();
    let action = args[0].clone();
    let filename = args[1].clone();

    // initialize the vm 
    let mut vm = vm::VM::new(&filename).unwrap();

    match &action[..] {
        "debug" => execute(&mut vm, true),
        "execute" => execute(&mut vm, false),
        _ => return
    }

}

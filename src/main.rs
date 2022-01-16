mod evm;
use evm::{
    vm, opcode::OpCode::EOF
};
use std::{
    env
};

fn execute(machine: &mut vm::VM, debug: bool) {
    loop {
        match machine.next() {
            Some(x) => if !debug && x != EOF {
                // for execution
                machine.step()
            } else if debug && x != EOF {
                // for debugging
                x.describe()
            } else {
                println!("reached the end of the line");
                return
            }
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

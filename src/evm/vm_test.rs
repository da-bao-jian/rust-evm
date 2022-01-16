use super::vm::VM;
use super::stack;
use web3::types::U256;
// initialize an instance of vm
fn create_vm(bytes: Vec<u8>) -> VM {
    VM {
        op_code: bytes,
        program_instruction: 0,
        stack: stack::Stack { lifo: Vec::new() }
    }
}

#[test]
fn test_vm_add(){

    let bytes = vec![0x60, 0x02, 0x60, 0x0f, 0x01, 0x00];
    let mut vm = create_vm(bytes);

    // push 0x02 to stack
    vm.step();
    assert_eq!(vm.program_instruction, 2);
    assert_eq!(vm.stack.lifo.len(), 1);
    assert_eq!(vm.stack.lifo[0], U256::from(2));

    // push 0x0f to the stack
    vm.step();
    assert_eq!(vm.program_instruction, 4);
    assert_eq!(vm.stack.lifo.len(), 2);
    assert_eq!(vm.stack.lifo[1], U256::from(15));

    // add 0x02 and 0x0f
    vm.step();
    assert_eq!(vm.program_instruction, 5);
    assert_eq!(vm.stack.lifo[0], U256::from(17));

}
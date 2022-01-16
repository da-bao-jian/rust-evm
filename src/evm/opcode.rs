#[allow(unused_variables)]
#[derive(PartialEq, Debug)]
pub enum OpCode {
    STOP(usize), // 0x00
    ADD(usize), // 0x01
    MUL(usize), // 0x02

    PUSH1(usize, u8), // 0x60
    PUSH2(usize, u8, u8), // 0x61
    PUSH32(usize, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // 0x7f 

    EOF,
}

impl OpCode {
    // function for debuging, which prints opcode to the terminal
    pub fn describe(&self) {
        match self {
            OpCode::STOP(line) => println!("0x{:x}\tSTOP\tHalts execution", line),
            OpCode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
            OpCode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
            OpCode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
            OpCode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),
            _ => println!("Unknown opcode")
        }
    }

}
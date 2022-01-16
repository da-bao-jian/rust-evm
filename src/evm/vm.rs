use crate::{
    evm::opcode::OpCode,
    evm::stack::Stack
};
use web3::types::U256;
use std::{
    fs,
    error::Error, 
    string::ParseError, 
};

fn hex_decoder(hex: &str) -> Result<Vec<u8>, ParseError> {
    let result: Vec<u8>;
    result = (0..hex.len()-1)
        .step_by(2)
        .map(
            |char_idx| u8::from_str_radix(&hex[char_idx..char_idx+2], 16)
            .unwrap()
        )
        .collect();
    Ok(result)
}

pub struct VM {
    // a vector of opcodes
    pub op_code: Vec<u8>,
    // current number of instruction
    pub program_instruction: usize, 
    pub stack: Stack,
}

#[allow(unused_variables)]
impl VM {
    pub fn new(filename: &str) -> Result<VM, Box<dyn Error>> {

        let parsed_string: String = fs::read_to_string(filename)?.parse()?;

        // read bytes
        let bytes = hex_decoder(&parsed_string[..])?;

        // return a instantiated VM
        Ok(VM{ op_code: bytes, program_instruction: 0, stack: Stack { lifo: Vec::new() }})
    }

    pub fn next(&mut self) -> Option<OpCode> {
        
        //prevent out of bound
        if self.program_instruction > self.op_code.len() {
            return Some(OpCode::EOF)
        }


        let addr = self.program_instruction;
        match self.op_code[self.program_instruction] {
            0x00 => {
                self.program_instruction += 1;
                Some(OpCode::STOP(self.program_instruction))
            }
            0x01 => {
                self.program_instruction += 1;
                Some(OpCode::ADD(self.program_instruction))
            },
            0x02 => {
                self.program_instruction += 1;
                Some(OpCode::MUL(self.program_instruction))
            },
            0x60 => {
                let value = self.op_code[self.program_instruction+1];
                self.program_instruction += 2;
                Some(OpCode::PUSH1(self.program_instruction,value))
            },
            0x61 => {
                let value0 = self.op_code[self.program_instruction+1];
                let value1 = self.op_code[self.program_instruction+2];
                self.program_instruction += 3;
                Some(OpCode::PUSH2(self.program_instruction, value0, value1))
            },
            _ => { 
                self.program_instruction += 1; 
                None
            },
        }
    }

    // printing the stack trace
    fn print_stack(&self) {

        self.stack.lifo
            .iter()
            .enumerate()
            .for_each(|(idx, val)| {
                let mut bytes = vec![0; 32];
                val.to_big_endian(&mut bytes);
                println!("{}: \t{:?}", idx, val)
            });

    }

    pub fn step(&mut self) {
        match self.next() {
            Some(x) => {
                match x {
                    OpCode::PUSH1(addr, value) =>  {
                        self.stack.lifo.push(U256::from(value));
                        self.print_stack()
                    }
                    OpCode::ADD(addr) => {
                        let top = self.stack.lifo.pop().unwrap();
                        let second = self.stack.lifo.pop().unwrap();

                        self.stack.lifo.push(top + second);
                        println!("Sum pushed on top of the stack");
                        self.print_stack()
                    }
                    // OpCode::MUL(_) => todo!(),
                    // OpCode::PUSH2(_, _, _) => todo!(),
                    // OpCode::PUSH32(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => todo!(),
                    // OpCode::EOF => todo!(),
                    _ => panic!("opcode not recognizable")
                }
            }
            None => return
            
        }
        self.print_stack()
    }

}
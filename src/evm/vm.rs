use crate::{evm::opcode::OpCode};
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

pub(crate) struct VM {
    hex_code: Vec<u8>,
    program_instruction: usize
}

#[allow(unused_variables)]
impl VM {
    pub fn new(filename: &str) -> Result<VM, Box<dyn Error>> {

        let parsed_string: String = fs::read_to_string(filename)?.parse()?;

        // read bytes
        let bytes = hex_decoder(&parsed_string[..])?;

        // return a instantiated VM
        Ok(VM{ hex_code: bytes, program_instruction: 0})
    }

    fn next(&mut self) -> Option<OpCode> {

        //prevent out of bound
        if self.program_instruction > self.hex_code.len() {
            return Some(OpCode::EOF)
        }

        let addr = self.program_instruction;
        match self.hex_code[self.program_instruction] {
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
                let value = self.hex_code[self.program_instruction+1];
                self.program_instruction += 2;
                Some(OpCode::PUSH1(self.program_instruction,value))
            },
            Ox61 => {
                let value0 = self.hex_code[self.program_instruction+1];
                let value1 = self.hex_code[self.program_instruction+2];
                self.program_instruction += 3;
                Some(OpCode::PUSH2(self.program_instruction, value0, value1))
            },
            _ => { self.program_instruction += 1; None}
    
        }
    }
    
}
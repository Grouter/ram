use std::collections::HashMap;

use crate::token::{Token, TokenType, TokenVal};

pub type InOpPair = (Instruction, Operand);

pub struct Instruction(String);

pub enum OperandType {
    Empty,
    Constant, 
    Register,
    Pointer
}

pub struct OperandVal(i32);

pub struct Operand {
    pub o_type: OperandType,
    pub val: OperandVal
}

pub fn parse(tokens: &Vec<Token>, labels: &HashMap<String, u32>) {
    let instruction_line: Vec<InOpPair> = Vec::new();

    let mut i = 0usize;

    while i < tokens.len() {
        let op = &tokens[i];

        if op.id != TokenType::Operation {
            panic!("Invalid token. Rogue value!")
        }

        if let TokenVal::Value(operation) = &op.value {

        }

        
    }
}
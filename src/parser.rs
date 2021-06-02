use std::collections::HashMap;
use std::fmt::Display;
use regex::Regex;

use crate::token::{Token, TokenType};

pub struct InOpPair {
    pub instruction: String,
    pub operand: Operand
}

pub enum Operand {
    Empty,
    Const(u32),
    Register(u32),
    Pointer(u32),
    Jump(u32)
}

impl Operand {
    pub fn to_number(&self) -> Result<u32, ()> {
        match self {
            Operand::Empty => Err(()),
            Operand::Const(n) =>    Ok(*n),
            Operand::Register(n) => Ok(*n),
            Operand::Pointer(n) =>  Ok(*n),
            Operand::Jump(n) =>     Ok(*n),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Empty => write!(f, "Empty"),
            Operand::Const(val) =>      write!(f, "{}", val),
            Operand::Register(val) =>   write!(f, "reg<{}>", val),
            Operand::Pointer(val) =>    write!(f, "p<{}>", val),
            Operand::Jump(val) =>       write!(f, "jump<{}>", val),
        }
    }
}

pub fn parse(tokens: &[Token]) -> Vec<InOpPair> {
    let mut instruction_line: Vec<InOpPair> = Vec::new();

    let register_id_pattern =   Regex::new(r"^[0-9]+").unwrap();
    let constant_pattern =      Regex::new(r"^=[0-9]+").unwrap();
    let pointer_pattern =       Regex::new(r"^*[0-9]+").unwrap();
    let label_pattern =         Regex::new(r"^[a-zA-Z]\w*").unwrap();

    fn get_number(s: &str) -> u32 {
        s.parse::<u32>().unwrap_or_else(|_| panic!("Invalid number: {}", s))
    }

    let mut labels: HashMap<String, u32> = HashMap::new();  // Label to instruction map
    let mut label_lookup: Vec<&String> = Vec::new();        // Label to Label ID (index)

    // Fetch all labels and create a label lookup "table". 
    // This only registers label and its ID (as index).
    // The real label to instruction mapping will be done in instruction decoding,
    // because we don't know yet how many instruction we will have and so we cannot map it properly here.
    for token in tokens {
        if token.id == TokenType::Label {
            label_lookup.push(&token.value);
        }
    }

    // Decode instructions
    let mut i = 0usize;
    while i < tokens.len() {
        let instruction_token = &tokens[i];

        // Create a label to instruction mapping.
        if instruction_token.id == TokenType::Label {
            labels.insert(instruction_token.value.to_owned(), instruction_line.len() as u32);
            i += 1;
            continue;
        }
        // If token is not an operation skip it. We can only parse operations.
        // This can be a possible bug or unwanted value, that would be caught by
        // a grammar check (which is not implemented yet).
        else if instruction_token.id != TokenType::Operation {
            i += 1;
            continue;
        }

        let instruction = instruction_token.value.to_owned();

        let operand_token = &tokens[i + 1];

        // If next token is a value token, pair it with the instruction as an operand.
        // Otherwise pair an empty operand.
        let operand = match operand_token.id {
            TokenType::Value => {
                let op: Operand;

                if register_id_pattern.is_match(&operand_token.value) {
                    op = Operand::Register(get_number(&operand_token.value));
                }
                else if constant_pattern.is_match(&operand_token.value) {
                    op = Operand::Const(get_number(operand_token.value.strip_prefix('=').unwrap()));
                }
                else if pointer_pattern.is_match(&operand_token.value) {
                    op = Operand::Pointer(get_number(operand_token.value.strip_prefix('*').unwrap()));
                }
                else if label_pattern.is_match(&operand_token.value) {
                    // Map jump instruction to label ID in lookup table. Not to the instruction index because we can 
                    // jump to an instruction before it is defined and direct mapping would cause an error.

                    let id_fetch = label_lookup.iter().position(|&l| l.eq(&operand_token.value));

                    if let Some(id) = id_fetch {
                        op = Operand::Jump(id as u32);
                    }
                    else {
                        panic!("Cannot register {} as a jump location", operand_token.value);
                    }
                }
                else {
                    panic!("Invalid operand {}", operand_token.value);
                }

                // Next token is already decoded. So skip it.
                i += 1;

                op
            }
            _ => Operand::Empty
        };

        instruction_line.push(InOpPair {
            instruction, 
            operand
        });

        // Move to the next token.
        i += 1;
    }

    // Now directly map all jump instructions to the instruction index.
    for i in &mut instruction_line {
        if let Operand::Jump(label_id) = i.operand {
            // Fetch label by ID.
            let label = label_lookup[label_id as usize];

            // Fetch the index from the label to instruction mapping.
            let instruction_index_fetch = labels.get(label);

            // Overwrite the jump ID by instruction index.
            if let Some(index) = instruction_index_fetch {
                i.operand = Operand::Jump(*index);
            }
        }
    }

    instruction_line
}

pub fn dump_instruction_line(line: &[InOpPair]) {
    println!("===== Instructions =====");
    for (i, in_op_pair) in line.iter().enumerate() {
        println!("[{}] {}: {}", i, in_op_pair.instruction, in_op_pair.operand);
    }
}
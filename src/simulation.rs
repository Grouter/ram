use std::io::stdin;
use std::time::Instant;

use crate::ProgramState;
use crate::operations::FUNCS_LOOKUP;
use crate::token::{Token, TokenVal};

pub fn simulate(tokens: &Vec<Token>, state: &mut ProgramState) {

    let mut ic= 0u32;
    let mut instructions = 0u32;
    let token_size = tokens.len() as u32;

    let now = Instant::now();

    while ic < token_size {
        let token = &tokens[ic as usize];

        match &token.id {
            crate::token::TokenType::Operation => {

                if let TokenVal::Value(operation) = &token.value {
                    match FUNCS_LOOKUP.get(operation.as_str()) {
                        Some(f) => {
                            instructions += 1;
                            (f)(&tokens, &mut ic, state);
                        },
                        None => println!("Unknown operation: {}", operation)
                    }
                }

            }
            _ => {}
        }

        ic += 1;

        /*let mut i = 0usize;
        for r in &state.registers {
            println!("[{}] {}", i, r);
            i += 1;
        }*/

        //let mut buff: String = String::new();
        //stdin().read_line(&mut buff).expect("ASD");
    }

    println!("==========================");
    println!("Evaluated {} instructions", instructions);
    println!("Duration {} micros", now.elapsed().as_micros());

    println!("INPUT:");
    let mut i = 0usize;
    for r in &state.input {
        println!("[{}] {}", i, r);
        i += 1;
    }

    println!("OUTPUT:");
    let mut i = 0usize;
    for r in &state.output {
        println!("[{}] {}", i, r);
        i += 1;
    }
}
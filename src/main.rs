#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs;

use simulation::simulate;
use token::tokenize;

use crate::token::Token;

pub const DEBUG_MODE: bool = false;

macro_rules! debug_log {
    ($($rest:tt)*) => {
        if crate::DEBUG_MODE {
            std::println!($($rest)*);
        }
    }
}

mod token;
mod parser;
mod simulation;
mod operations;

pub struct ProgramState {
    pub input: Vec<i32>,
    pub input_pointer: usize,
    pub output: Vec<i32>,
    pub registers: Vec<i32>,
    pub labels: HashMap<String, u32>
}

fn main() {
    let contents = fs::read_to_string("./instructions.ram")
        .expect("Something went wrong reading the file");

    let mut state = ProgramState {
        input: vec![4],
        input_pointer: 0,
        output: Vec::new(),
        registers: vec![0; 5],
        labels: HashMap::new()
    };

    let tokens: Vec<Token> = tokenize(&contents, &mut state.labels);

    //simulate(&tokens, &mut state);
}

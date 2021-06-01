#[macro_use]
extern crate lazy_static;

use std::fs;

use simulation::simulate;
use parser::parse;
use token::tokenize;

pub const DEBUG_MODE: bool = false;
const REGISTER_COUNT: usize = 5;

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
    pub ic: u32,
    pub input: Vec<i32>,
    pub input_pointer: usize,
    pub output: Vec<i32>,
    pub registers: Vec<i32>
}

fn main() {
    let contents = fs::read_to_string("./instructions.ram")
        .expect("Something went wrong while reading the RAM file");

    let mut state = ProgramState {
        ic: 0,
        input: vec![4],
        input_pointer: 0,
        output: Vec::new(),
        registers: vec![0; REGISTER_COUNT]
    };

    let tokens = tokenize(&contents);
    let line = parse(&tokens);

    simulate(&line, &mut state);
}

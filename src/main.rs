#[macro_use]
extern crate lazy_static;

use std::fs;

use simulation::simulate;
use parser::{dump_instruction_line, parse};
use token::{dump_tokens, tokenize};

pub const   VERBOSE_MODE: bool =            true;
pub const   DUMP_REGISTERS: bool =          false;
const       DUMP_TOKENS: bool =             false;
const       DUMP_ISNTRUCTION_LINE: bool =   false;
pub const   STEP_DEBUG: bool =              false;

const REGISTER_COUNT: usize = 5;

macro_rules! debug_log {
    ($($rest:tt)*) => {
        if crate::VERBOSE_MODE {
            std::println!($($rest)*);
        }
    }
}

mod token;
mod parser;
mod simulation;
mod operations;

pub struct ProgramState {
    pub output: Vec<i32>,                   // Output Tape
    pub input_pointer: usize,               // Input Head
    pub input: [i32; 1],                    // Input Tape
    pub ic: u32,                            // Instruction Counter
    pub registers: [i32; REGISTER_COUNT],   // Registers
}

fn main() {
    let contents = fs::read_to_string("./instructions.ram")
        .expect("Something went wrong while reading the RAM file");

    let mut state = ProgramState {
        ic: 0,
        input: [4; 1],
        input_pointer: 0,
        output: Vec::new(),
        registers: [0; REGISTER_COUNT]
    };

    let tokens = tokenize(&contents);

    if DUMP_TOKENS {
        dump_tokens(&tokens);
    }

    let line = parse(&tokens);

    if DUMP_ISNTRUCTION_LINE {
        dump_instruction_line(&line);
    }

    simulate(&line, &mut state);
}

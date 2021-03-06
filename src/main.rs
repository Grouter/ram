#[macro_use]
extern crate lazy_static;

use std::fs;

use simulation::simulate;
use parser::{dump_instruction_line, parse};
use token::{dump_tokens, tokenize};

pub const   VERBOSE_MODE: bool =            true;   // Logs every instruction
pub const   DUMP_REGISTERS: bool =          false;  // Dumps registers after each instruction
const       DUMP_TOKENS: bool =             false;  // Dumps tokens after tokenizing
const       DUMP_INSTRUCTION_LINE: bool =   false;  // Dumps instruction line after parsing
pub const   STEP_DEBUG: bool =              false;  // Wait for user ENTER after each instruction

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

// Input needs to be a Vec because user will want to resize this at runtime later.
pub struct ProgramState {
    pub output: Vec<i32>,                   // Output Tape
    pub input: Vec<i32>,                    // Input Tape
    pub input_pointer: usize,               // Input Head
    pub ic: u32,                            // Instruction Counter
    pub registers: [i32; REGISTER_COUNT],   // Registers
    pub exit_state: bool                    // Should program exit
}

fn main() {
    let contents = fs::read_to_string("./instructions.ram")
        .expect("Something went wrong while reading the RAM file");

    let mut state = ProgramState {
        ic: 0,
        input: vec![4; 1],
        input_pointer: 0,
        output: Vec::new(),
        registers: [0; REGISTER_COUNT],
        exit_state: false
    };

    let tokens = tokenize(&contents);

    if DUMP_TOKENS {
        dump_tokens(&tokens);
    }

    let line = parse(&tokens);

    if DUMP_INSTRUCTION_LINE {
        dump_instruction_line(&line);
    }

    simulate(&line, &mut state);
}

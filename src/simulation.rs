use std::io::stdin;
use std::time::Instant;

use crate::ProgramState;
use crate::parser::InstructionLine;
use crate::operations::FUNCS_LOOKUP;

pub fn simulate(line: &InstructionLine, state: &mut ProgramState) {

    let mut evaluated_instructions = 0u32;
    let line_size = line.len() as u32;

    let now = Instant::now();

    while state.ic < line_size {
        let in_op_pair = &line[state.ic as usize];

        // Fetch a function that handles the instrution.
        match FUNCS_LOOKUP.get(in_op_pair.instruction.as_str()) {
            Some(f) => {
                evaluated_instructions += 1;
                (f)(&in_op_pair.operand, state);
            },
            None => println!("Unknown instruction: {}", in_op_pair.instruction)
        }

        state.ic += 1;

        /*let mut i = 0usize;
        for r in &state.registers {
            println!("[{}] {}", i, r);
            i += 1;
        }

        let mut buff: String = String::new();
        stdin().read_line(&mut buff).expect("Err");*/
    }

    println!("==========================");
    println!("Evaluated {} instructions", evaluated_instructions);
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
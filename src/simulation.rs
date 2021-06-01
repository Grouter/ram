use std::io::stdin;
use std::time::Instant;

use crate::{ProgramState, DUMP_REGISTERS, STEP_DEBUG, VERBOSE_MODE};
use crate::parser::InstructionLine;
use crate::operations::FUNCS_LOOKUP;

pub fn simulate(line: &InstructionLine, state: &mut ProgramState) {

    if VERBOSE_MODE {
        println!("===== Simulation =====")
    }

    let mut evaluated_instructions = 0u32;
    let line_size = line.len() as u32;

    let program_timer = Instant::now();
    let mut instruction_timer: Instant;

    let mut instruction_duration_micros_sum = 0u128;

    while state.ic < line_size {
        let in_op_pair = &line[state.ic as usize];

        // Fetch a function that handles the instruction.
        match FUNCS_LOOKUP.get(in_op_pair.instruction.as_str()) {
            Some(f) => {
                evaluated_instructions += 1;

                instruction_timer = Instant::now();
                (f)(&in_op_pair.operand, state);
                instruction_duration_micros_sum += instruction_timer.elapsed().as_micros();
            },
            None => println!("Unknown instruction: {}", in_op_pair.instruction)
        }

        state.ic += 1;

        if DUMP_REGISTERS {
            let mut i = 0usize;
            for r in &state.registers {
                println!("[{}] {}", i, r);
                i += 1;
            }
        }

        if STEP_DEBUG {

            let mut buff: String = String::new();
            stdin().read_line(&mut buff).expect("Err");
        }
    }

    println!("===== Result =====");
    println!("Evaluated {} instructions", evaluated_instructions);
    println!("Duration {} micros", program_timer.elapsed().as_micros());
    println!(
        "Average instruction duration: {} micros",
        instruction_duration_micros_sum as f32 / evaluated_instructions as f32
    );

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
use std::collections::HashMap;

use crate::ProgramState;
use crate::token::Token;

pub type OpFn = fn(&Vec<Token>, &mut u32, &mut ProgramState);

lazy_static! {
    pub static ref FUNCS_LOOKUP: HashMap<&'static str, OpFn> = {
        let mut map = HashMap::new();
        map.insert("LOAD", load as OpFn);
        map.insert("STORE", store as OpFn);
        map.insert("READ", read as OpFn);
        map.insert("ADD", add as OpFn);
        map.insert("SUB", sub as OpFn);
        map.insert("MUL", mul as OpFn);
        map.insert("DIV", div as OpFn);
        map.insert("DEFINE", define as OpFn);
        map.insert("JZERO", jzero as OpFn);
        map.insert("JUMP", jump as OpFn);
        map.insert("WRITE", write as OpFn);
        map
    };
}

fn fetch_value(token: &Token, registers: &Vec<i32>) -> Option<i32> {
    let value = token.value.to_number();

    match token.id {
        crate::token::TokenType::Register => {
            Some(registers[value as usize])
        }
        crate::token::TokenType::Constant => {
            Some(value as i32)
        }
        crate::token::TokenType::Pointer => {
            let register = registers[value as usize];

            Some(registers[register as usize])
        }
        _ => None
    }
}

pub fn load(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    match fetch_value(operand, &state.registers) {
        Some(value) => {
            debug_log!("[LOAD] {} to ACC({})", value, state.registers[0]);
            state.registers[0] = value;
        }
        None => panic!("Invlaid operand type for LOAD.")
    }
}

pub fn read(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    let register = operand.value.to_number() as usize;

    if state.input_pointer >= state.input.len() {
        debug_log!("[READ] INPUT_C is out of bounds... Reading 0.");
        state.registers[register] = 0;
    }
    else {
        debug_log!("[READ] Read {} to register {}", state.input[state.input_pointer], register);

        state.registers[register] = state.input[state.input_pointer];
        state.input_pointer += 1;
    }

}

pub fn store(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    let register = operand.value.to_number() as usize;

    debug_log!("[STORE] {} to register {}", state.registers[0], register);
    state.registers[register] = state.registers[0];
}

pub fn add(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    match fetch_value(operand, &state.registers) {
        Some(value) => {
            debug_log!("[ADD] {} to ACC({})", value, state.registers[0]);
            state.registers[0] += value;
        }
        None => panic!("Invlaid operand type for ADD.")
    }
}

pub fn sub(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    match fetch_value(operand, &state.registers) {
        Some(value) => {
            debug_log!("[SUB] {} from ACC({})", value, state.registers[0]);
            state.registers[0] -= value;
        }
        _ => panic!("Invlaid operand type for SUB.")
    }
}

pub fn mul(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    match fetch_value(operand, &state.registers) {
        Some(value) => {
            debug_log!("[MUL] ACC({}) by {}", state.registers[0], value);
            state.registers[0] *= value;
        }
        _ => panic!("Invlaid operand type for MUL.")
    }
}

pub fn div(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    match fetch_value(operand, &state.registers) {
        Some(value) => {
            debug_log!("[DUV] ACC({}) by {}", state.registers[0], value);

            assert!(value != 0, "Attempted to divide by zero");

            state.registers[0] /= value;
        }
        _ => panic!("Invlaid operand type for DIV.")
    }
}

pub fn define(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    debug_log!("[DEFINE] Inserting new label {} to ic {}", operand.value.to_string(), *ic + 1);
    state.labels.insert(operand.value.to_value(), *ic + 1);
}

pub fn jzero(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    let target_ic = match state.labels.get(&operand.value.to_value()) {
        Some(ic) => ic,
        None => panic!("Cannot jump to an invalid label!")
    };

    if state.registers[0] == 0 {
        debug_log!("[JZERO] Jumping to {} on ic {}", operand.value.to_value(), *target_ic);
        *ic = *target_ic;
    }
}

pub fn jump(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    let target_ic = match state.labels.get(&operand.value.to_value()) {
        Some(ic) => ic,
        None => panic!("Cannot jump to an invalid label!")
    };

    debug_log!("[JUMP] Jumping to {} on ic {}", operand.value.to_value(), *target_ic);
    *ic = *target_ic;
}

pub fn write(tokens: &Vec<Token>, ic: &mut u32, state: &mut ProgramState) {
    *ic += 1;
    let operand = &tokens[*ic as usize];

    match fetch_value(operand, &state.registers) {
        Some(value) => {
            debug_log!("[WRITE] {}", value);

            state.output.push(value);

            state.registers[0] /= value;
        }
        _ => panic!("Invlaid operand type for WRITE.")
    }
}
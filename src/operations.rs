use std::collections::HashMap;

use crate::ProgramState;
use crate::parser::Operand;

pub type OpFn = fn(&Operand, &mut ProgramState);

lazy_static! {
    pub static ref FUNCS_LOOKUP: HashMap<&'static str, OpFn> = {
        let mut map = HashMap::new();
        map.insert("LOAD",   load as OpFn);
        map.insert("STORE",  store as OpFn);
        map.insert("READ",   read as OpFn);
        map.insert("ADD",    add as OpFn);
        map.insert("SUB",    sub as OpFn);
        map.insert("MUL",    mul as OpFn);
        map.insert("DIV",    div as OpFn);
        map.insert("JZERO",  jzero as OpFn);
        map.insert("JGZERO", jgzero as OpFn);
        map.insert("JUMP",   jump as OpFn);
        map.insert("WRITE",  write as OpFn);
        map.insert("HALT",   halt as OpFn);
        map
    };
}

// TODO: there is a lot of boiler plate for each operation...

fn fetch(operand: &Operand, registers: &[i32]) -> i32 {
    match operand {
        Operand::Const(n) => *n as i32,
        Operand::Register(n) => registers[*n as usize],
        Operand::Pointer(n) => {
            let register = registers[*n as usize];

            registers[register as usize]
        },
        _ => {
            panic!("Cannot fetch a value from {}", operand);
        }
    }
}

pub fn load(operand: &Operand, state: &mut ProgramState) {
    let value = fetch(operand, &state.registers);

    debug_log!("[LOAD] {} to ACC", value);

    state.registers[0] = value;
}

pub fn read(operand: &Operand, state: &mut ProgramState) {

    let register = operand.to_number()
        .expect("READ needs a numerical operand") as usize;

    if state.input_pointer >= state.input.len() {
        debug_log!("[READ] input tape head is out of bounds... Reading 0.");
        state.registers[register] = 0;
    }
    else {
        debug_log!("[READ] {} to register {}", state.input[state.input_pointer], register);

        state.registers[register] = state.input[state.input_pointer];
        state.input_pointer += 1;
    }
}

pub fn store(operand: &Operand, state: &mut ProgramState) {
    let register = operand.to_number()
        .expect("STORE needs a numerical operand") as usize;

    debug_log!("[STORE] {} to register {}", state.registers[0], register);
    
    state.registers[register] = state.registers[0];
}

pub fn add(operand: &Operand, state: &mut ProgramState) {
    let value = fetch(operand, &state.registers);

    debug_log!("[ADD] {} to ACC({})", value, state.registers[0]);
    
    state.registers[0] += value
}

pub fn sub(operand: &Operand, state: &mut ProgramState) {
    let value = fetch(operand, &state.registers);

    debug_log!("[SUB] {} from ACC({})", value, state.registers[0]);
    
    state.registers[0] -= value
}

pub fn mul(operand: &Operand, state: &mut ProgramState) {
    let value = fetch(operand, &state.registers);

    debug_log!("[MUL] {} by ACC({})", value, state.registers[0]);
    
    state.registers[0] *= value
}

pub fn div(operand: &Operand, state: &mut ProgramState) {
    let value = fetch(operand, &state.registers);

    debug_log!("[DIV] ACC({}) by {}", value, state.registers[0]);
    
    state.registers[0] /= value
}

pub fn jzero(operand: &Operand, state: &mut ProgramState) {
    let index = operand.to_number().unwrap();

    if state.registers[0] == 0 {
        debug_log!("[JZERO] Jumping to {}", index);

        state.ic = index - 1;
    }
    else {
        debug_log!("[JZERO] Condition not met");
    }
}

pub fn jgzero(operand: &Operand, state: &mut ProgramState) {
    let index = operand.to_number().unwrap();

    if state.registers[0] > 0 {
        debug_log!("[JGZERO] Jumping to {}", index);

        state.ic = index - 1;
    }
    else {
        debug_log!("[JZERO] Condition not met");
    }
}

pub fn jump(operand: &Operand, state: &mut ProgramState) {
    let index = operand.to_number().unwrap();

    debug_log!("[JUMP] Jumping to {}", index);
    
    state.ic = index - 1;
}

pub fn write(operand: &Operand, state: &mut ProgramState) {
    let value = fetch(operand, &state.registers);

    debug_log!("[WRITE] {}", value);

    state.output.push(value);
}

pub fn halt(operand: &Operand, state: &mut ProgramState) {
    state.exit_state = true;
}
# Random Access Machine

## TODO
* [ ] HALT
* [ ] JGZERO
* [ ] Grammars
* [ ] TUI and editor
* [ ] Complexity calculation (logarithmic)

## Before Use (Dependencies)
* Download [Rust](https://www.rust-lang.org/)

## Use
Currently file loading is hardcoded to load the `instructions.ram` file.

1) To try out the functionality either
    * modify the `instructions.ram` file
    * rewrite the hardcoded value in `main.rs` to load other file.

2) Run the program:
```bash
cargo run
```

To get more verbose outputs modify these values in `main.rs`
```rust
pub const   VERBOSE_MODE: bool =            true;
pub const   DUMP_REGISTERS: bool =          false;
const       DUMP_TOKENS: bool =             false;
const       DUMP_INSTRUCTION_LINE: bool =   false;
pub const   STEP_DEBUG: bool =              false;
```

## Commands

| Instruction | Operand | Description                                         |
|-------------|---------|-----------------------------------------------------|
| LOAD        | reg     | Load value from register to ACC                     |
| STORE       | reg     | Store value from ACC to register                    |
| STORE       | *reg    | Store value from ACC to register (indirectly)       |
| ADD         | =const  | Adds value to ACC                                   |
| SUB         | =const  | Subtracts value from ACC                            |
| MUL         | =const  | Multiply ACC by a value                             |
| DIV         | =const  | Divide ACC by a value                               |
| READ        | reg     | Read value from input tape to register              |
| READ        | *reg    | Read value from input tape to register (indirectly) |
| WRITE       | reg     | Writes a value from register to the output tape     |
| JUMP        | label   | Jumps to a label                                    |
| JZERO      | label   | Jumps to a label (if ACC == 0)                      |

#### Explanations
1) `reg` = Index of a register
```   
    For example to multiple ACC by a value stored in the register 1 use MUL 1
    
    ACC is refering to a register 0
```
2) `*reg` = Indirect linking. That means the value stored in a register with a index `reg` will be used as a index.
```
    For example:
   
   | Register | Value |
   |----------|-------|
   | 0 (ACC)  | 0     |
   | 1        | 10    |
   | 2        | 1     |
   
    The instruction LOAD *2 will look into the register 2
    The register 2 has a value of 1, so the register 1 (value of 10) will be loaded to ACC
```
3) `const` = Working with pure constants.
```    
    For example to multiply ACC by 42 use MUL =42
```
4) `label` = Label name
```
    For example:
    
    AWESOME_LABEL:      <=== Define label
    ... some code ...
    JUMP AWESOME_LABEL  <=== Jump to the label
    
    You can also jump to the label before it is defined
```
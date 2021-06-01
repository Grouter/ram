use std::collections::HashMap;
use std::fmt::Display;

use regex::Regex;

const PATTERN: &str = r"([A-Z]+)( ?)(=?[0-9]+|[a-zA-Z]+|:|)";

#[derive(PartialEq, Eq)]
pub enum TokenType {
    Label, 
    Operation, 
    Constant, 
    Register, 
    Pointer,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TokenType::Label => write!(f, "Label"),
            TokenType::Operation => write!(f, "Operation"),
            TokenType::Constant => write!(f, "Constant"),
            TokenType::Register => write!(f, "Register"),
            TokenType::Pointer => write!(f, "Pointer"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum TokenVal {
    Value(String),
    Number(u32),
}

impl TokenVal {
    pub fn to_number(&self) -> u32 {
        let value = match self {
            TokenVal::Number(n) => n,
            _ => panic!("Invlaid operand value for LOAD.")
        };

        *value
    }

    pub fn to_value(&self) -> String {
        let value = match self {
            TokenVal::Value(n) => n.to_string(),
            _ => panic!("Invlaid operand value for LOAD.")
        };

        value
    }
}

impl Display for TokenVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenVal::Value(value) => write!(f, "{}", value),
            TokenVal::Number(num) => write!(f, "{}", num),
        }
    }
}

pub struct Token {
    pub id: TokenType,
    pub value: TokenVal
}

pub fn tokenize(content: &str, labels: &mut HashMap<String, u32>) -> Vec<Token> {
    let token_pattern = Regex::new(PATTERN).unwrap();

    let register_id_pattern = Regex::new(r"^[0-9]+").unwrap();
    let constant_pattern = Regex::new(r"^=[0-9]+").unwrap();
    let pointer_pattern = Regex::new(r"^*[0-9]+").unwrap();

    let mut tokens: Vec<Token> = Vec::new();

    let captures = token_pattern.captures_iter(content);

    fn get_number(s: &str) -> u32 {
        s.parse::<u32>().expect("Invalid Number")
    }

    for token in captures {
        let a = token.get(1).unwrap().as_str().to_owned();
        let b = token.get(3).unwrap().as_str().to_owned();

        assert_eq!(token.len(), 4, "Unrecognized token sequence!");

        if b.eq(":") {
            debug_log!("Registering a new label {} at {}", a, tokens.len() - 1);
            labels.insert(a, tokens.len() as u32 - 1);
        }
        else if b.is_empty() {
            tokens.push(Token {
                id: TokenType::Operation,
                value: TokenVal::Value(a)
            });
        }
        else {
            tokens.push(Token {
                id: TokenType::Operation,
                value: TokenVal::Value(a)
            });
            
            if register_id_pattern.is_match(&b) {
                tokens.push(Token {
                    id: TokenType::Register,
                    value: TokenVal::Number(get_number(&b))
                });
            }
            else if constant_pattern.is_match(&b) {
                tokens.push(Token {
                    id: TokenType::Constant,
                    value: TokenVal::Number(get_number(b.strip_prefix("=").unwrap()))
                });
            }
            else if pointer_pattern.is_match(&b) {
                tokens.push(Token {
                    id: TokenType::Pointer,
                    value: TokenVal::Number(get_number(b.strip_prefix("*").unwrap()))
                });
            }
            else {
                tokens.push(Token {
                    id: TokenType::Label,
                    value: TokenVal::Value(b)
                });
            }
        }
    }

    for t in &tokens {
        println!("{}", t);
    }

    tokens
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n\tID: {}\n\tVALUE: {}\n}}", self.id, self.value)
    }
}
use std::fmt::Display;

use regex::Regex;

const PATTERN: &str = r"([A-Z]+)( ?)(=?[0-9]+|[a-zA-Z]+|:|)";

#[derive(PartialEq, Eq)]
pub enum TokenType {
    Operation, 
    Value,
    Label,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TokenType::Label =>     write!(f, "Label"),
            TokenType::Operation => write!(f, "Operation"),
            TokenType::Value =>     write!(f, "Value"),
        }
    }
}

pub struct Token {
    pub id: TokenType,
    pub value: String
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.value)
    }
}

pub fn tokenize(content: &str) -> Vec<Token> {
    let token_pattern = Regex::new(PATTERN).unwrap();
    
    let mut tokens: Vec<Token> = Vec::new();

    let captures = token_pattern.captures_iter(content);

    for token in captures {
        let a = token.get(1).unwrap().as_str().to_owned();
        let b = token.get(3).unwrap().as_str().to_owned();

        assert_eq!(token.len(), 4, "Unrecognized token sequence!");

        if b.eq(":") {
            tokens.push(Token {
                id: TokenType::Label,
                value: a
            });
        }
        else {
            tokens.push(Token {
                id: TokenType::Operation,
                value: a
            });

            if !b.is_empty() {
                tokens.push(Token {
                    id: TokenType::Value,
                    value:b
                });
            }
        }
    }

    tokens
}

pub fn dump_tokens(tokens: &Vec<Token>) {
    println!("===== Tokens =====");
    for t in tokens {
        println!("{}", t);
    }
}
use crate::tokens::{Token, TokenType};
use std::collections::HashMap;

pub struct Compiler {
    tokens: Vec<Token>,
    label_positions: HashMap<String, usize>,
}

impl Compiler {
    pub fn new(tokens: Vec<Token>) -> Compiler {
        Compiler {
            tokens,
            label_positions: HashMap::new(),
        }
    }

    pub fn compile_instructions(&mut self) -> Vec<u8> {
        let mut instructions = Vec::new();
        let mut num_instructions = 0;

        for tok in &self.tokens {
            if *tok.token() == TokenType::LOADLABEL {
                self.label_positions.insert(tok.lexeme(), num_instructions);
            }

            instructions.push(tok.to_bytes());
            num_instructions += 1;
        }

        instructions
    }
}
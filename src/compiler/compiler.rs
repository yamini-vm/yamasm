use crate::tokens::Token;

pub struct Compiler {
    tokens: Vec<Token>,
}

impl Compiler {
    pub fn new(tokens: Vec<Token>) -> Compiler {
        Compiler {
            tokens,
        }
    }

    pub fn compile_instructions(&self) -> Vec<u8> {
        let mut instructions = Vec::new();

        for tok in &self.tokens {
            instructions.push(tok.to_bytes());
        }

        instructions
    }
}
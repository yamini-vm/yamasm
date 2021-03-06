use std::collections::HashMap;

use crate::tokens::{Token, TokenType};
use super::constants::{REGISTER_OFFSET, STACK_OFFSET, STACK_OFFSET_STR, DATA_MEMORY_OFFSET};
use super::constants::{ADDR_OFFSET, PTR_OFFSET};

pub struct Compiler {
    tokens: Vec<Token>,
    current_position: usize,
}

impl Compiler {
    pub fn new(tokens: Vec<Token>) -> Compiler {
        Compiler {
            tokens,
            current_position: 0,
        }
    }

    fn get_next_token(&mut self) -> &Token {
        self.current_position += 1;
        let token = &self.tokens[self.current_position];
        token
    }

    fn get_current_token(&self) -> &Token {
        &self.tokens[self.current_position]
    }

    fn peek_next_token(&self) -> &Token {
        &self.tokens[self.current_position + 1]
    }

    fn expect_next_token(&mut self, token_types: Vec<TokenType>) {
        let next_token = self.peek_next_token();

        let mut found = false;
        let mut expected_token_types = String::new();
        for token_type in token_types {
            if next_token.token() == &token_type {
                found = true;
                break;
            } else {
                expected_token_types.push_str(&format!("{:?} ", token_type));
            }
        }

        if !found {
            panic!("Expected {:?} but found {:?}", expected_token_types, next_token);
        }
    }

    pub fn compile_instructions(&mut self) -> Vec<u8> {
        let mut instructions = Vec::new();
        let mut num_instructions = 0;

        let mut label_positions = HashMap::new();
        let mut backpatch_store = HashMap::new();

        // Pass 1: Compile instructions
        while self.current_position < self.tokens.len() {
            let current_token = self.get_current_token();

            match current_token.token() {
                TokenType::LOAD => {
                    instructions.append(&mut current_token.to_bytes());
                    self.expect_next_token(vec![TokenType::NUM, TokenType::REG, TokenType::STR,
                                                           TokenType::VAR, TokenType::ADDR]);
                    let next_token = self.get_next_token();

                    match next_token.token() {
                        TokenType::NUM => {
                            instructions.push(Some(STACK_OFFSET));
                        },
                        TokenType::STR => {
                            instructions.push(Some(STACK_OFFSET_STR));
                        },
                        TokenType::REG => {
                            instructions.push(Some(REGISTER_OFFSET));
                        },
                        TokenType::VAR => {
                            instructions.push(Some(DATA_MEMORY_OFFSET));
                        },
                        TokenType::ADDR => {
                            instructions.push(Some(ADDR_OFFSET));
                        },
                        _ => {
                            panic!("Expected register, number or string");
                        }
                    }
                    
                    instructions.append(&mut next_token.to_bytes());
                },
                TokenType::ADD | TokenType::SUB | TokenType::MUL | TokenType::DIV | TokenType::MOD
                | TokenType::HALT | TokenType::STR | TokenType::SHOW
                | TokenType::RET | TokenType::NEG | TokenType::EQU | TokenType::DEREF => {
                    instructions.append(&mut current_token.to_bytes());
                },
                TokenType::JMP | TokenType::JZ | TokenType::JN | TokenType::CALL => {
                    instructions.append(&mut current_token.to_bytes());
                    self.expect_next_token(vec![TokenType::LABEL]);
                    let next_token = self.get_next_token();
                    instructions.push(None);
                    backpatch_store.insert(instructions.len() - 1, next_token.lexeme());
                },
                TokenType::LABEL => {
                    instructions.append(&mut current_token.to_bytes());
                    label_positions.insert(current_token.lexeme(), num_instructions);
                }
                TokenType::NUM | TokenType::REG | TokenType::STARTSTR | TokenType::ENDSTR | TokenType::VAR
                | TokenType::ADDR | TokenType::PTR => {
                    panic!("Unexpected token {:?}", current_token);
                },
                TokenType::POP => {
                    instructions.append(&mut current_token.to_bytes());
                    self.expect_next_token(vec![TokenType::REG, TokenType::VAR, TokenType::PTR]);
                    let next_token = self.get_next_token();

                    match next_token.token() {
                        TokenType::REG => {
                            instructions.push(Some(REGISTER_OFFSET));
                        },
                        TokenType::VAR => {
                            instructions.push(Some(DATA_MEMORY_OFFSET));
                        },
                        TokenType::PTR => {
                            instructions.push(Some(PTR_OFFSET));
                        },
                        _ => {
                            panic!("Expected register, variable or pointer");
                        }
                    }

                    instructions.append(&mut next_token.to_bytes());

                },
            }

            num_instructions += 1;
            self.current_position += 1;
        }

        // Pass 2: Backpatch jumps
        for (idx, label_name) in backpatch_store {
            instructions[idx] = Some(label_positions[&label_name] as u8);
        }

        let mut definitive_instructions = Vec::new();

        for instruction in instructions {
            definitive_instructions.push(instruction.unwrap());
        }

        definitive_instructions
    }
}
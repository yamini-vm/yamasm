use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::tokens::{Token, TokenType};

enum State {
    START,
    READ,
    SKIP,
}

pub struct Lexer {
    state: State,
    lexeme: String,
    reading_string: bool,
    current_str: String,
}

fn read_file_line_by_line(filepath: &str) -> Result<Vec<char>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut file_chars = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                for ch in line.chars() {
                    file_chars.push(ch);
                }
            },
            Err(e) => println!("Error: {}", e),
        }
        file_chars.push('\n');
    }

    Ok(file_chars)
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            state: State::START,
            lexeme: String::new(),
            reading_string: false,
            current_str: String::new(),
        }
    }

    fn isspace(&self, chr: char) -> bool {
        match chr {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false,
        }
    }

    fn tokenize_lexeme(&mut self, tokens: &mut Vec<Token>) {
        let keywords = ["load", "add", "sub", "mul", "div", "ret", "mod", "jmp",
                                 "pop", "jz", "jn"];

        let word = self.lexeme.to_lowercase();

        if keywords.contains(&word.as_str()) {
            let token_type = match TokenType::from(&word) {
                Some(token_type) => token_type,
                None => panic!("Unknown token type: {}", word),
            };
            tokens.push(Token::new(token_type, self.lexeme.clone()));
        } else if word.starts_with("%") {
            let label_name: String = self.lexeme.as_str()[1..].to_string();
            let label_token = Token::new(TokenType::LABEL, label_name.clone());
            tokens.push(label_token);
        } else if word.starts_with("r") {
            let register_idx = &word[1..];
            let register_token = Token::new(TokenType::REG, register_idx.to_string());
            tokens.push(register_token);
        } else if word.starts_with("\"") {
            let string_start_token = Token::new(TokenType::STARTSTR, "STARTSTR".to_string());
            tokens.push(string_start_token);

            self.current_str = self.lexeme[1..].to_string().clone();
            self.current_str.push_str(" ");

            self.reading_string = true;
        } else if word.ends_with("\"") {
            self.current_str.push_str(self.lexeme[..self.lexeme.len() - 1].to_string().clone().as_str());

            let string_token = Token::new(TokenType::STR, self.current_str.clone());
            tokens.push(string_token);

            let string_end_token = Token::new(TokenType::ENDSTR, "ENDSTR".to_string());
            tokens.push(string_end_token);

            self.reading_string = false;
            self.current_str = String::new();
        }
        else {
            if self.reading_string {
                self.current_str.push_str(self.lexeme.clone().as_str());
                self.current_str.push_str(" ");
            } else {
                let token_type = TokenType::NUM;
                let token = Token::new(token_type, self.lexeme.clone());
                tokens.push(token);
            }
        }
    }

    pub fn tokenize(&mut self, file_path: &str) -> Vec<Token> {
        let chars = read_file_line_by_line(file_path).unwrap();
        let mut tokens = Vec::new();

        let mut i = 0;
        while i < chars.len() {
            match self.state {
                State::START => {
                    if self.isspace(chars[i]) {
                        self.state = State::SKIP;
                    } else {
                        self.state = State::READ;
                    }
                },
                State::READ => {
                    if self.isspace(chars[i]) {
                        self.state = State::SKIP;
                    } else {
                        self.lexeme.push(chars[i]);
                        self.state = State::READ;
                        i += 1;
                    }
                },
                State::SKIP => {
                    if self.isspace(chars[i]) {
                        self.state = State::SKIP;
                        i += 1;
                    } else {
                        self.tokenize_lexeme(&mut tokens);
                        self.lexeme.clear();
                        self.state = State::START;
                    }
                },
            }
        }

        if !self.lexeme.is_empty() {
            self.tokenize_lexeme(&mut tokens);
            self.lexeme.clear();
        }

        tokens
    }
}
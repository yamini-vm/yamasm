use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::tokens::{Token, TokenType};

enum State {
    START,
    READ,
    READSTR,
    SKIP,
}

pub struct Lexer {
    state: State,
    lexeme: String,
    reading_string: bool,
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
        }
        else {
            let token_type;
            if self.reading_string {
                token_type = TokenType::STR;
                let start_string_token = Token::new(TokenType::STARTSTR, "startstr".to_string());
                tokens.push(start_string_token);
            } else {
                token_type = TokenType::NUM;
            }

            let token = Token::new(token_type, self.lexeme.clone());
            tokens.push(token);

            if self.reading_string {
                let end_string_token = Token::new(TokenType::ENDSTR, "endstr".to_string());
                tokens.push(end_string_token);
                self.reading_string = false;
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
                    } else if chars[i] == '"' {
                        self.state = State::READSTR;
                        i += 1;
                    } else {
                        self.lexeme.push(chars[i]);
                        self.state = State::READ;
                        i += 1;
                    }
                },
                State::READSTR => {
                    let mut j = i;

                    while chars[j] != '"' {
                        self.lexeme.push(chars[j]);
                        j += 1;
                    }
                    j += 1; // Skip the closing quote
                    i = j;
                    self.reading_string = true;
                    self.state = State::START;
                }
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
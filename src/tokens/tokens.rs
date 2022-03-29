#[derive(Debug)]
pub enum TokenType {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    RET,
    NUM,
}

impl TokenType {
    pub fn from(token: &str) -> Option<TokenType> {
        match token {
            "load" => Some(TokenType::LOAD),
            "add" => Some(TokenType::ADD),
            "sub" => Some(TokenType::SUB),
            "mul" => Some(TokenType::MUL),
            "div" => Some(TokenType::DIV),
            "ret" => Some(TokenType::RET),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    token: TokenType,
    lexeme: String,
}

impl Token {
    pub fn new(token: TokenType, lexeme: String) -> Token {
        Token {
            token,
            lexeme,
        }
    }
}
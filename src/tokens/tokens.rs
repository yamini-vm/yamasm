#[derive(Debug)]
pub enum TokenType {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    RET,
    NUM,
    MOD,
    LABEL,
    JMP,
    POP,
    JZ,
    JN,
    REG,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TokenType::LOAD, TokenType::LOAD) => true,
            (TokenType::ADD, TokenType::ADD) => true,
            (TokenType::SUB, TokenType::SUB) => true,
            (TokenType::MUL, TokenType::MUL) => true,
            (TokenType::DIV, TokenType::DIV) => true,
            (TokenType::RET, TokenType::RET) => true,
            (TokenType::NUM, TokenType::NUM) => true,
            (TokenType::MOD, TokenType::MOD) => true,
            (TokenType::LABEL, TokenType::LABEL) => true,
            (TokenType::JMP, TokenType::JMP) => true,
            (TokenType::POP, TokenType::POP) => true,
            (TokenType::JZ, TokenType::JZ) => true,
            (TokenType::JN, TokenType::JN) => true,
            (TokenType::REG, TokenType::REG) => true,
            _ => false,
        }
    }
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
            "mod" => Some(TokenType::MOD),
            "jmp" => Some(TokenType::JMP),
            "pop" => Some(TokenType::POP),
            "jz" => Some(TokenType::JZ),
            "jn" => Some(TokenType::JN),
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

    pub fn token(&self) -> &TokenType {
        &self.token
    }

    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }

    pub fn to_bytes(&self) -> u8 {
        match self.token {
            TokenType::LOAD => 0,
            TokenType::ADD => 1,
            TokenType::SUB => 2,
            TokenType::MUL => 3,
            TokenType::DIV => 4,
            TokenType::RET => 5,
            TokenType::NUM => self.lexeme.parse::<u8>().unwrap(),
            TokenType::MOD => 6,
            TokenType::LABEL => 7,
            TokenType::JMP => 8,
            TokenType::POP => 9,
            TokenType::JZ => 10,
            TokenType::JN => 11,
            TokenType::REG => self.lexeme.parse::<u8>().unwrap(),
        }
    }
}
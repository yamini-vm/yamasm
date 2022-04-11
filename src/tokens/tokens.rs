#[derive(Debug)]
pub enum TokenType {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HALT,
    NUM,
    MOD,
    LABEL,
    JMP,
    POP,
    JZ,
    JN,
    REG,
    STARTSTR,
    ENDSTR,
    STR,
    SHOW,
    RET,
    CALL,
    EQU,
    VAR,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TokenType::LOAD, TokenType::LOAD) => true,
            (TokenType::ADD, TokenType::ADD) => true,
            (TokenType::SUB, TokenType::SUB) => true,
            (TokenType::MUL, TokenType::MUL) => true,
            (TokenType::DIV, TokenType::DIV) => true,
            (TokenType::HALT, TokenType::HALT) => true,
            (TokenType::NUM, TokenType::NUM) => true,
            (TokenType::MOD, TokenType::MOD) => true,
            (TokenType::LABEL, TokenType::LABEL) => true,
            (TokenType::JMP, TokenType::JMP) => true,
            (TokenType::POP, TokenType::POP) => true,
            (TokenType::JZ, TokenType::JZ) => true,
            (TokenType::JN, TokenType::JN) => true,
            (TokenType::REG, TokenType::REG) => true,
            (TokenType::STARTSTR, TokenType::STARTSTR) => true,
            (TokenType::ENDSTR, TokenType::ENDSTR) => true,
            (TokenType::STR, TokenType::STR) => true,
            (TokenType::SHOW, TokenType::SHOW) => true,
            (TokenType::RET, TokenType::RET) => true,
            (TokenType::CALL, TokenType::CALL) => true,
            (TokenType::EQU, TokenType::EQU) => true,
            (TokenType::VAR, TokenType::VAR) => true,
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
            "halt" => Some(TokenType::HALT),
            "mod" => Some(TokenType::MOD),
            "jmp" => Some(TokenType::JMP),
            "pop" => Some(TokenType::POP),
            "jz" => Some(TokenType::JZ),
            "jn" => Some(TokenType::JN),
            "show" => Some(TokenType::SHOW),
            "ret" => Some(TokenType::RET),
            "call" => Some(TokenType::CALL),
            "equ" => Some(TokenType::EQU),
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

    pub fn to_bytes(&self) -> Vec<Option<u8>> {
        match self.token {
            TokenType::LOAD => vec![Some(0)],
            TokenType::ADD => vec![Some(1)],
            TokenType::SUB => vec![Some(2)],
            TokenType::MUL => vec![Some(3)],
            TokenType::DIV => vec![Some(4)],
            TokenType::HALT => vec![Some(5)],
            TokenType::NUM => vec![Some(self.lexeme.parse::<u8>().unwrap())],
            TokenType::MOD => vec![Some(6)],
            TokenType::LABEL => vec![Some(7)],
            TokenType::JMP => vec![Some(8)],
            TokenType::POP => vec![Some(9)],
            TokenType::JZ => vec![Some(10)],
            TokenType::JN => vec![Some(11)],
            TokenType::REG => vec![Some(self.lexeme.parse::<u8>().unwrap())],
            TokenType::STARTSTR => vec![Some(12)],
            TokenType::ENDSTR => vec![Some(13)],
            TokenType::STR => {
                let mut bytes = Vec::new();
                for c in self.lexeme.chars() {
                    bytes.push(Some(c as u8));
                }
                bytes
            },
            TokenType::SHOW => vec![Some(14)],
            TokenType::RET => vec![Some(15)],
            TokenType::CALL => vec![Some(16)],
            TokenType::EQU => vec![Some(17)],
            TokenType::VAR => vec![Some(self.lexeme.parse::<u8>().unwrap())],
        }
    }
}
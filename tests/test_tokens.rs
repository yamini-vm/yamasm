use yamasm::tokens::{TokenType, Token};

#[test]
fn test_token_type_equality() {
    let token_type = TokenType::LOAD;
    assert_eq!(token_type, TokenType::LOAD);

    let token_type = TokenType::ADD;
    assert_eq!(token_type, TokenType::ADD);

    let token_type = TokenType::SUB;
    assert_eq!(token_type, TokenType::SUB);

    let token_type = TokenType::MUL;
    assert_eq!(token_type, TokenType::MUL);

    let token_type = TokenType::DIV;
    assert_eq!(token_type, TokenType::DIV);

    let token_type = TokenType::HALT;
    assert_eq!(token_type, TokenType::HALT);

    let token_type = TokenType::NUM;
    assert_eq!(token_type, TokenType::NUM);

    let token_type = TokenType::MOD;
    assert_eq!(token_type, TokenType::MOD);

    let token_type = TokenType::LABEL;
    assert_eq!(token_type, TokenType::LABEL);

    let token_type = TokenType::JMP;
    assert_eq!(token_type, TokenType::JMP);

    let token_type = TokenType::POP;
    assert_eq!(token_type, TokenType::POP);

    let token_type = TokenType::JZ;
    assert_eq!(token_type, TokenType::JZ);

    let token_type = TokenType::JN;
    assert_eq!(token_type, TokenType::JN);

    let token_type = TokenType::STARTSTR;
    assert_eq!(token_type, TokenType::STARTSTR);

    let token_type = TokenType::ENDSTR;
    assert_eq!(token_type, TokenType::ENDSTR);

    let token_type = TokenType::STR;
    assert_eq!(token_type, TokenType::STR);

    let token_type = TokenType::SHOW;
    assert_eq!(token_type, TokenType::SHOW);

    let token_type = TokenType::RET;
    assert_eq!(token_type, TokenType::RET);

    let token_type = TokenType::CALL;
    assert_eq!(token_type, TokenType::CALL);

    let token_type = TokenType::EQU;
    assert_eq!(token_type, TokenType::EQU);

    let token_type = TokenType::VAR;
    assert_eq!(token_type, TokenType::VAR);

    let token_type = TokenType::NEG;
    assert_eq!(token_type, TokenType::NEG);

    let token_type = TokenType::ADDR;
    assert_eq!(token_type, TokenType::ADDR);

    let token_type = TokenType::DEREF;
    assert_eq!(token_type, TokenType::DEREF);
}

#[test]
fn test_token_from() {
    let token_type = TokenType::from("load");
    assert_eq!(token_type, Some(TokenType::LOAD));

    let token_type = TokenType::from("add");
    assert_eq!(token_type, Some(TokenType::ADD));

    let token_type = TokenType::from("sub");
    assert_eq!(token_type, Some(TokenType::SUB));

    let token_type = TokenType::from("mul");
    assert_eq!(token_type, Some(TokenType::MUL));

    let token_type = TokenType::from("div");
    assert_eq!(token_type, Some(TokenType::DIV));

    let token_type = TokenType::from("halt");
    assert_eq!(token_type, Some(TokenType::HALT));

    let token_type = TokenType::from("mod");
    assert_eq!(token_type, Some(TokenType::MOD));

    let token_type = TokenType::from("jmp");
    assert_eq!(token_type, Some(TokenType::JMP));

    let token_type = TokenType::from("pop");
    assert_eq!(token_type, Some(TokenType::POP));

    let token_type = TokenType::from("jz");
    assert_eq!(token_type, Some(TokenType::JZ));

    let token_type = TokenType::from("jn");
    assert_eq!(token_type, Some(TokenType::JN));

    let token_type = TokenType::from("show");
    assert_eq!(token_type, Some(TokenType::SHOW));

    let token_type = TokenType::from("ret");
    assert_eq!(token_type, Some(TokenType::RET));

    let token_type = TokenType::from("call");
    assert_eq!(token_type, Some(TokenType::CALL));

    let token_type = TokenType::from("equ");
    assert_eq!(token_type, Some(TokenType::EQU));

    let token_type = TokenType::from("neg");
    assert_eq!(token_type, Some(TokenType::NEG));

    let token_type = TokenType::from("_");
    assert_eq!(token_type, None);
}

#[test]
fn test_token_token_fn() {
    let token = Token::new(TokenType::ADD, "add".to_string());
    assert_eq!(token.token(), &TokenType::ADD);
}

#[test]
fn test_token_lexeme_fn() {
    let token = Token::new(TokenType::ADD, "add".to_string());
    assert_eq!(token.lexeme(), "add");
}

#[test]
fn test_token_to_bytes() {
    let token = Token::new(TokenType::LOAD, "load".to_string());
    assert_eq!(token.to_bytes(), vec![Some(0)]);

    let token = Token::new(TokenType::ADD, "add".to_string());
    assert_eq!(token.to_bytes(), vec![Some(1)]);

    let token = Token::new(TokenType::SUB, "sub".to_string());
    assert_eq!(token.to_bytes(), vec![Some(2)]);

    let token = Token::new(TokenType::MUL, "mul".to_string());
    assert_eq!(token.to_bytes(), vec![Some(3)]);

    let token = Token::new(TokenType::DIV, "div".to_string());
    assert_eq!(token.to_bytes(), vec![Some(4)]);

    let token = Token::new(TokenType::HALT, "ret".to_string());
    assert_eq!(token.to_bytes(), vec![Some(5)]);

    let token = Token::new(TokenType::NUM, "6".to_string());
    assert_eq!(token.to_bytes(), vec![Some(6)]);

    let token = Token::new(TokenType::MOD, "mod".to_string());
    assert_eq!(token.to_bytes(), vec![Some(6)]);

    let token = Token::new(TokenType::LABEL, "loadlabel".to_string());
    assert_eq!(token.to_bytes(), vec![Some(7)]);

    let token = Token::new(TokenType::JMP, "jmp".to_string());
    assert_eq!(token.to_bytes(), vec![Some(8)]);

    let token = Token::new(TokenType::POP, "popreg".to_string());
    assert_eq!(token.to_bytes(), vec![Some(9)]);

    let token = Token::new(TokenType::JZ, "jz".to_string());
    assert_eq!(token.to_bytes(), vec![Some(10)]);

    let token = Token::new(TokenType::JN, "jn".to_string());
    assert_eq!(token.to_bytes(), vec![Some(11)]);

    let token = Token::new(TokenType::STARTSTR, "startstr".to_string());
    assert_eq!(token.to_bytes(), vec![Some(12)]);

    let token = Token::new(TokenType::ENDSTR, "endstr".to_string());
    assert_eq!(token.to_bytes(), vec![Some(13)]);

    let token = Token::new(TokenType::STR, "Hello".to_string());
    assert_eq!(token.to_bytes(), vec![Some(72), Some(101), Some(108), Some(108), Some(111)]);

    let token = Token::new(TokenType::SHOW, "show".to_string());
    assert_eq!(token.to_bytes(), vec![Some(14)]);

    let token = Token::new(TokenType::RET, "ret".to_string());
    assert_eq!(token.to_bytes(), vec![Some(15)]);

    let token = Token::new(TokenType::CALL, "call".to_string());
    assert_eq!(token.to_bytes(), vec![Some(16)]);

    let token = Token::new(TokenType::EQU, "equ".to_string());
    assert_eq!(token.to_bytes(), vec![Some(17)]);

    let token = Token::new(TokenType::NEG, "neg".to_string());
    assert_eq!(token.to_bytes(), vec![Some(18)]);

    let token = Token::new(TokenType::NUM, "0".to_string());
    assert_eq!(token.to_bytes(), vec![Some(0)]);

    let token = Token::new(TokenType::ADDR, "1".to_string());
    assert_eq!(token.to_bytes(), vec![Some(1)]);

    let token = Token::new(TokenType::DEREF, "deref".to_string());
    assert_eq!(token.to_bytes(), vec![Some(19)]);
}
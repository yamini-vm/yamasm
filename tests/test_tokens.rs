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

    let token_type = TokenType::RET;
    assert_eq!(token_type, TokenType::RET);

    let token_type = TokenType::NUM;
    assert_eq!(token_type, TokenType::NUM);

    let token_type = TokenType::MOD;
    assert_eq!(token_type, TokenType::MOD);

    let token_type = TokenType::LOADLABEL;
    assert_eq!(token_type, TokenType::LOADLABEL);

    let token_type = TokenType::JMP;
    assert_eq!(token_type, TokenType::JMP);

    let token_type = TokenType::LOADREGISTER;
    assert_eq!(token_type, TokenType::LOADREGISTER);

    let token_type = TokenType::POPREGISTER;
    assert_eq!(token_type, TokenType::POPREGISTER);
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

    let token_type = TokenType::from("ret");
    assert_eq!(token_type, Some(TokenType::RET));

    let token_type = TokenType::from("mod");
    assert_eq!(token_type, Some(TokenType::MOD));

    let token_type = TokenType::from("jmp");
    assert_eq!(token_type, Some(TokenType::JMP));

    let token_type = TokenType::from("loadreg");
    assert_eq!(token_type, Some(TokenType::LOADREGISTER));

    let token_type = TokenType::from("popreg");
    assert_eq!(token_type, Some(TokenType::POPREGISTER));

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
    assert_eq!(token.to_bytes(), 0);

    let token = Token::new(TokenType::ADD, "add".to_string());
    assert_eq!(token.to_bytes(), 1);

    let token = Token::new(TokenType::SUB, "sub".to_string());
    assert_eq!(token.to_bytes(), 2);

    let token = Token::new(TokenType::MUL, "mul".to_string());
    assert_eq!(token.to_bytes(), 3);

    let token = Token::new(TokenType::DIV, "div".to_string());
    assert_eq!(token.to_bytes(), 4);

    let token = Token::new(TokenType::RET, "ret".to_string());
    assert_eq!(token.to_bytes(), 5);

    let token = Token::new(TokenType::NUM, "6".to_string());
    assert_eq!(token.to_bytes(), 6);

    let token = Token::new(TokenType::MOD, "mod".to_string());
    assert_eq!(token.to_bytes(), 6);

    let token = Token::new(TokenType::LOADLABEL, "loadlabel".to_string());
    assert_eq!(token.to_bytes(), 7);

    let token = Token::new(TokenType::JMP, "jmp".to_string());
    assert_eq!(token.to_bytes(), 8);

    let token = Token::new(TokenType::LOADREGISTER, "loadreg".to_string());
    assert_eq!(token.to_bytes(), 9);

    let token = Token::new(TokenType::POPREGISTER, "popreg".to_string());
    assert_eq!(token.to_bytes(), 10);
}
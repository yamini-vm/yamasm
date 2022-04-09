use yamasm::lexer::Lexer;
use yamasm::tokens::TokenType;

#[test]
fn test_tokenize_lexeme() {
    let mut lexer = Lexer::new();

    let tokens = lexer.tokenize("tests/data/test.yas");

    assert_eq!(tokens.len(), 11);

    assert_eq!(tokens[0].token(), &TokenType::LOAD);
    assert_eq!(tokens[0].lexeme(), "LOAD");

    assert_eq!(tokens[1].token(), &TokenType::NUM);
    assert_eq!(tokens[1].lexeme(), "4");

    assert_eq!(tokens[2].token(), &TokenType::LOAD);
    assert_eq!(tokens[2].lexeme(), "LOAD");

    assert_eq!(tokens[3].token(), &TokenType::NUM);
    assert_eq!(tokens[3].lexeme(), "5");

    assert_eq!(tokens[4].token(), &TokenType::ADD);
    assert_eq!(tokens[4].lexeme(), "ADD");

    assert_eq!(tokens[5].token(), &TokenType::LABEL);
    assert_eq!(tokens[5].lexeme(), "SUB");

    assert_eq!(tokens[6].token(), &TokenType::LOAD);
    assert_eq!(tokens[6].lexeme(), "LOAD");

    assert_eq!(tokens[7].token(), &TokenType::NUM);
    assert_eq!(tokens[7].lexeme(), "2");

    assert_eq!(tokens[8].token(), &TokenType::SUB);
    assert_eq!(tokens[8].lexeme(), "SUB");

    assert_eq!(tokens[9].token(), &TokenType::LABEL);
    assert_eq!(tokens[9].lexeme(), "END");

    assert_eq!(tokens[10].token(), &TokenType::HALT);
    assert_eq!(tokens[10].lexeme(), "HALT");
}
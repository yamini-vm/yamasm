use yamasm::compiler::Compiler;
use yamasm::lexer::Lexer;

#[test]
fn test_compile_instructions() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("tests/data/test.yas");

    let mut compiler = Compiler::new(tokens);
    let instructions = compiler.compile_instructions();

    assert_eq!(instructions.len(), 14);

    let expected_instructions: Vec<u8> = vec![
        0,
        2,
        4,
        0,
        2,
        5,
        1,
        7,
        0,
        2,
        2,
        2,
        7,
        5,
    ];

    for (i, instruction) in instructions.iter().enumerate() {
        assert_eq!(instruction, &expected_instructions[i]);
    }
}
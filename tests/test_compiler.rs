use yamasm::compiler::Compiler;
use yamasm::lexer::Lexer;

#[test]
fn test_compile_instructions() {
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize("tests/data/test.yas");

    let mut compiler = Compiler::new(tokens);
    let instructions = compiler.compile_instructions();

    assert_eq!(instructions.len(), 11);

    assert_eq!(instructions[0], 0);
    assert_eq!(instructions[1], 4);
    assert_eq!(instructions[2], 0);
    assert_eq!(instructions[3], 5);
    assert_eq!(instructions[4], 1);
    assert_eq!(instructions[5], 7);
    assert_eq!(instructions[6], 0);
    assert_eq!(instructions[7], 2);
    assert_eq!(instructions[8], 2);
    assert_eq!(instructions[9], 7);
    assert_eq!(instructions[10], 5);
}
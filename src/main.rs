use yamasm::lexer::Lexer;
use yamasm::compiler::Compiler;

fn main() {
    let mut lexer = Lexer::new();

    let tokens = lexer.tokenize("test.yas");
    
    let compiler = Compiler::new(tokens);
}
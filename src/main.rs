use yamasm::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new();

    let tokens = lexer.tokenize("test.yas");
    for token in tokens {
        println!("{:?}", token);
    }
}
use std::fs::{OpenOptions, self};
use std::io::prelude::*;
use std::path::Path;

use yamasm::lexer::Lexer;
use yamasm::compiler::Compiler;

fn main() {
    let mut lexer = Lexer::new();

    let tokens = lexer.tokenize("test.yas");
    
    let compiler = Compiler::new(tokens);
    
    let instructions = compiler.compile_instructions();

    let file_name = "a.out";

    if Path::new(file_name).exists() {
        fs::remove_file(file_name).unwrap();
      }

    let mut out_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(file_name)
        .unwrap();

    match out_file.write_all(&instructions) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
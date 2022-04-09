use std::fs::{OpenOptions, self};
use std::io::prelude::*;
use std::path::Path;
use std::env;

use yamasm::lexer::Lexer;
use yamasm::compiler::Compiler;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filepath> [<binpath>]", args[0]);
        return;
    }
    let file_path = &args[1];
    let bin_path = if args.len() > 2 {
        &args[2]
    } else {
        "a.out"
    };

    let mut lexer = Lexer::new();

    let tokens = lexer.tokenize(&file_path);
    
    let mut compiler = Compiler::new(tokens);
    
    let instructions = compiler.compile_instructions();

    if instructions[instructions.len() - 1] != 5 {
        println!("Error: last instruction must be HALT");
        return;
    }

    if Path::new(bin_path).exists() {
        fs::remove_file(bin_path).unwrap();
    }

    let mut out_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(bin_path)
        .unwrap();

    match out_file.write_all(&instructions) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
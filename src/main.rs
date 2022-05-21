extern crate sargparse;

use std::fs::{OpenOptions, self};
use std::io::prelude::*;
use std::path::Path;
use sargparse::{ArgumentParser, ArgumentType, InnerData};

use yamasm::lexer::Lexer;
use yamasm::compiler::Compiler;

fn main() {
    let mut parser = ArgumentParser::new(Some("YamASM - Assembler for YaminiVM"));

    parser.add_argument("f", "file_path", "File path to yas file", 
                        true, None, ArgumentType::STR);
    parser.add_argument("-o", "--output", "Output binary file path", 
                        false, Some(InnerData::STR("a.out".to_string())), ArgumentType::STR);
    parser.add_argument("-t", "--tokens", "Flag to print tokens",
                        false, Some(InnerData::BOOL(false)), ArgumentType::BOOL);
    parser.add_argument("-i", "--instructions", "Flag to print compiled instructions",
                        false, Some(InnerData::BOOL(false)), ArgumentType::BOOL);

    let args = parser.parse_args().unwrap();

    let file_path = args.get("file_path").unwrap().get_str();
    let bin_path = &args.get("output").unwrap().get_str();

    let token_flag = args.get("tokens").unwrap().get_bool();
    let instructions_flag = args.get("instructions").unwrap().get_bool();

    let mut lexer = Lexer::new();

    let tokens = lexer.tokenize(&file_path);

    if token_flag {
        println!("--------------------------------------------");
        println!("Tokens:");
        for token in &tokens {
            println!("{:?}", token);
        }
        println!("--------------------------------------------");
    }

    let mut compiler = Compiler::new(tokens);
    
    let instructions = compiler.compile_instructions();

    if instructions_flag {
        println!("--------------------------------------------");
        println!("Instructions:");
        for instruction in &instructions {
            println!("{:?}", instruction);
        }
        println!("--------------------------------------------");
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
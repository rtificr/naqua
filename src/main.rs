use std::fs;
use crate::parse::parser::Parser;
use crate::runtime::runtime::Runner;
use crate::tokenize::token::Token;
use crate::tokenize::tokenizer::Tokenizer;

mod util;
mod tokenize;
mod parse;
mod runtime;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Incorrect usage!");
        println!("Correct usage: naqua <filename>");
        return;
    }
    let input = fs::read_to_string(args.get(1).unwrap()).unwrap();

    let mut tokenizer = Tokenizer::new(input);
    match tokenizer.tokenize() {
        Ok(tokens) => {
            let mut parser = Parser::new(&tokens);
            match parser.parse() {
                Ok(ast) => {
                    let mut rt = Runner::new();
                    match rt.run(ast) {
                        Ok(_) => {}
                        Err(e) => println!("Runtime error: {e}")
                    }
                }
                Err(e) => println!("Parsing error: {e}")
            }
        }
        Err(e) => println!("Tokenization error: {e}")
    }
    println!("\n\nDone!");
}

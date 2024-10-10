use std::fs;
use std::path::Path;
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
    let input;

    let mut should_log = args.contains(&String::from("-l"));

    if args.len() < 2 || args.len() > 3 {
        if Path::new("test.naq").exists() {
            input = fs::read_to_string("test.naq").unwrap();
        } else {
            println!("Incorrect usage!");
            println!("Correct usage: naqua <filename> <flags>");
            return;
        }
    } else {
        input = fs::read_to_string(args.get(1).unwrap()).unwrap();
    }
    
    should_log = true;

    println!();

    let mut tokenizer = Tokenizer::new(input);
    match tokenizer.tokenize() {
        Ok(tokens) => {
            if should_log { println!("Tokens: \n{:?}\n", tokens); }
            let mut parser = Parser::new(&tokens, should_log);
            match parser.parse() {
                Ok(ast) => {
                    if should_log {
                        for node in &ast {
                            println!("{:?}", node);
                        }
                        println!();
                        println!("Running...");
                        println!();
                    }
                    let mut rt = Runner::new();
                    match rt.run(ast) {
                        Ok(_) => {}
                        Err(e) => eprintln!("Runtime error: {e}")
                    }
                }
                Err(e) => eprintln!("Parsing error: {e}")
            }
        }
        Err(e) => eprintln!("Tokenization error: {e}")
    }
}

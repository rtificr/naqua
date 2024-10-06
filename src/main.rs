use std::fs;
use crate::parse::parser::Parser;
use crate::runtime::runtime::Runner;
use crate::tokenize::tokenizer::Tokenizer;

mod util;
mod tokenize;
mod parse;
mod runtime;

fn main() {
    let input = fs::read_to_string("test.nqa").unwrap();

    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize().unwrap();

    println!("{tokens:?}");
    
    let mut parser = Parser::new(&tokens);
    let ast = parser.parse().unwrap();
    
    println!("{ast:?}");
    
    for node in &ast { 
        println!("{node:?}")
    }
    
    println!();
    
    let mut rt = Runner::new();
    rt.run(ast).unwrap();

    println!();
    
}

use std::fs;
use std::path::Path;
use crate::parse::parser::Parser;
use crate::runtime::runtime::Runner;
use crate::tokenize::tokenizer::Tokenizer;

pub fn poll() -> Result<(), &'static str>{
    let args = std::env::args().collect::<Vec<String>>();
    let input;

    let mut should_log = args.contains(&String::from("-l"));

    if !(2..3).contains(&args.len()) {
        if Path::new("test.naq").exists() {
            input = fs::read_to_string("test.naq").unwrap();
        } else {
            println!("Incorrect usage!");
            println!("Correct usage: naqua <filename> <flags>");
            return Err("Incorrect usage!");
        }
    } else {
        input = fs::read_to_string(args.get(1)
            .ok_or("No path found! If you need to run it without arguments, \
                add a file 'test.naq' in Naqua's root directory!")?)
            .map_err(|_| "Failed to read file!")?;
    }

    // should_log = true;

    println!();

    let mut tokenizer = Tokenizer::new(input);
    match tokenizer.tokenize() {
        Ok(tokens) => {
            if should_log { println!("Tokens: \n{:?}\n", tokens); }
            let mut parser = Parser::new(&tokens, should_log);
            match parser.parse() {
                Ok(ast) => {
                    if should_log {
                        for mac in &ast.macros {
                            println!("MACRO {:?}", mac);
                        }
                        for node in &ast.nodes {
                            println!("{:?}", node);
                        }
                        if should_log { println!(); }
                    }
                    let mut rt = Runner::new();
                    match rt.run(ast.nodes, ast.macros) {
                        Ok(_) => {}
                        Err(e) => eprintln!("Runtime error: {e}")
                    }
                }
                Err(e) => eprintln!("Parsing error: {e}")
            }
        }
        Err(e) => eprintln!("Tokenization error: {e}")
    }
    println!();
    Ok(())
}
use std::fs;
use std::path::Path;
use crate::parse::parser::Parser;
use crate::poll::poll;
use crate::runtime::runtime::Runner;
use crate::tokenize::token::Token;
use crate::tokenize::tokenizer::Tokenizer;

mod util;
mod tokenize;
mod parse;
mod runtime;
mod poll;

fn main() {
    match poll() {
        Ok(()) => return,
        Err(e) => eprintln!("Error: {e}")
    }
}

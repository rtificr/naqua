use std::collections::HashMap;
use crate::util::types::{Number, Operator};

pub mod parser;
mod keyword_parse;
mod num_parse;
mod rel_parse;

#[derive(Debug, Clone)]
pub enum Node {
    Literal(Number),
    Char(Box<Node>),
    Print(Box<Node>),
    If(Box<Node>, Box<Vec<Node>>),
    Loop(Box<Vec<Node>>),
    Assign(Box<Node>, Box<Node>),
    Eval(Box<Node>, Operator, Box<Node>),
    Define(String, Box<Vec<Node>>),
    Think(Box<Node>),
    Out(Box<Node>),
    Run(String),
    Spawn(String),
    Break
}
impl Node {
    pub fn to_num(&self) -> Result<Number, &'static str> {
        match self {
            Node::Literal(n) => Ok(*n),
            _ => Err("Unable to convert a non-number node to a number!")
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprType {
    Node(Option<Node>),
    Macro((String, Box<Vec<Node>>)),
}
pub struct ParserResult {
    pub(crate) nodes: Vec<Node>,
    pub(crate) macros: Option<HashMap<String, Box<Vec<Node>>>>
}
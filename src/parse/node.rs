use crate::util::types::*;

#[derive(Debug, Clone)]
pub enum Node {
    Literal(Number),
    Char(Box<Node>),
    Print(Box<Node>),
    If(Box<Node>, Box<Vec<Node>>),
    Loop(Box<Vec<Node>>),
    Assign(Box<Node>, Box<Node>),
    Eval(Box<Node>, Operator, Box<Node>),
    Think(Box<Node>),
    Out(Box<Node>),
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
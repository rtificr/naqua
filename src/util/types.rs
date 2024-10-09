use std::fmt::{Display, Formatter};
use crate::runtime::eval::EvalType;
use crate::util::types::Keyword::Print;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f32),
    Thought
}
impl Number {
    pub fn thought(&self) -> bool {
        if let Number::Thought = self { true } else { false }
    }
    pub fn int(&self) -> Option<i64> {
        if let Number::Int(n) = self { Some(*n) } else { None }
    }
    pub fn float(&self) -> Option<f32> {
        if let Number::Float(n) = self { Some(*n) } 
        else if let Number::Int(n) = self { Some(*n as f32) } 
        else { None }
    }
    pub fn eval_type(&self) -> Option<EvalType> {
        match self {
            Number::Int(n) => Some(EvalType::Int(*n)), 
            Number::Float(n) => Some(EvalType::Float(*n)), 
            _ => None
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
impl Operator {
    pub fn from(c: char) -> Operator {
        match c {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '*' => Operator::Mul,
            '/' => Operator::Div,
            _ => Operator::Add
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            Operator::Add => '+',
            Operator::Sub => '-',
            Operator::Mul => '*',
            Operator::Div => '/'
        }
    }
}
impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Think,
    Thought,
    In,
    Out,
    Print,
    If,
    Loop,
    Break,
    Char,
}
impl Keyword {
    pub fn from(input: &str) -> Option<Keyword> {
        match input {
            "think" => Some(Keyword::Think),
            "thought" => Some(Keyword::Thought),
            "print" => Some(Keyword::Print),
            "if" => Some(Keyword::If),
            "loop" => Some(Keyword::Loop),
            "break" => Some(Keyword::Break),
            "in" => Some(Keyword::In),
            "out" => Some(Keyword::Out),
            "char" => Some(Keyword::Char),
            _ => None
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            Keyword::Think => "think",
            Keyword::Thought => "thought",
            Keyword::In => "in",
            Keyword::Out => "out",
            Keyword::Print => "print",
            Keyword::If => "if",
            Keyword::Loop => "loop",
            Keyword::Break => "break",
            Keyword::Char => "char",
        }
    }
}
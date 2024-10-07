use std::ops::Deref;
use crate::parse::node::Node;
use crate::runtime::eval::EvalType;
use crate::runtime::runtime::Runner;
use crate::util::types::{Number, Operator, Operator::*};

impl Runner {
    pub fn op_eval(&self, lhs: Box<Node>, op: Operator, rhs: Box<Node>) -> EvalType {
        let l = match lhs.deref() {
            Node::Literal(n) => match n {
                Number::Thought => self.thought,
                _ => *n
            },
            _ => Number::Int(0)
        };
        let r = match rhs.deref() {
            Node::Literal(n) => match n {
                Number::Thought => self.thought,
                _ => *n
            },
            _ => Number::Int(0)
        };

        if l.int().is_some() && r.int().is_some() {
            match op {
                Add => EvalType::Int(l.int().unwrap() + r.int().unwrap()),
                Sub => EvalType::Int(l.int().unwrap() - r.int().unwrap()),
                Mul => EvalType::Int(l.int().unwrap() * r.int().unwrap()),
                Div => EvalType::Float(l.float().unwrap() / r.float().unwrap()),
            }
        } else {
            match op {
                Add => EvalType::Float(l.float().unwrap() + r.float().unwrap()),
                Sub => EvalType::Float(l.float().unwrap() - r.float().unwrap()),
                Mul => EvalType::Float(l.float().unwrap() * r.float().unwrap()),
                Div => EvalType::Float(l.float().unwrap() / r.float().unwrap()),
            }
        }
    }
}
use std::ops::Deref;
use crate::parse::Node;
use crate::runtime::eval::EvalType;
use crate::runtime::runtime::Runner;
use crate::util::types::{Number, Operator, Operator::*};

impl Runner {
    pub fn op_eval(&self, lhs: Box<Node>, op: Operator, rhs: Box<Node>) -> EvalType {
        let eval_node = |node: &Node| -> Number {
            match node {
                Node::Out(n) => self.eval(&Node::Out(n.clone())).to_num().unwrap(),
                Node::Literal(n) => match n {
                    Number::Thought => self.thought,
                    _ => *n
                },
                Node::Eval(l, o, r) => 
                    self.op_eval(
                        l.clone(), 
                        *o, 
                        r.clone()
                    ).to_num().unwrap(),
                _ => Number::Int(0),
            }
        };

        let l = eval_node(lhs.deref());
        let r = eval_node(rhs.deref());

        if l.is_int() && r.is_int() {
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
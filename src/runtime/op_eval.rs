use std::ops::Deref;
use crate::parse::Node;
use crate::runtime::eval::EvalType;
use crate::runtime::runtime::Runner;
use crate::util::types::{Number, Operator, Operator::*};

impl Runner {
    pub fn op_eval(&mut self, lhs: Box<Node>, op: Operator, rhs: Box<Node>) -> Result<EvalType, String> {
        let mut eval_node = |node: &Node| -> Result<Number, String> {
            match node {
                Node::Out(n) => Ok(self.eval(&Node::Out(n.clone()))?.to_num().unwrap()),
                Node::Literal(n) => match n {
                    Number::Thought => Ok(self.thought),
                    _ => Ok(*n)
                },
                Node::Eval(l, o, r) =>
                    Ok(self.op_eval(
                        l.clone(),
                        *o,
                        r.clone(),
                    )?.to_num().unwrap()),
                _ => Ok(Number::Int(0))
            }
        };

        let l = eval_node(lhs.deref())?;
        let r = eval_node(rhs.deref())?;

        Ok(if l.is_int() && r.is_int() {
            match op {
                Add => EvalType::Int(l.int().unwrap() + r.int().unwrap()),
                Sub => EvalType::Int(l.int().unwrap() - r.int().unwrap()),
                Mul => EvalType::Int(l.int().unwrap() * r.int().unwrap()),
                Div => {
                    if r.float() == 0. { return Err(self.err("Attempted division by zero!")); }
                    EvalType::Float(l.float() / r.float())
                }
            }
        } else {
            match op {
                Add => EvalType::Float(l.float() + r.float()),
                Sub => EvalType::Float(l.float() - r.float()),
                Mul => EvalType::Float(l.float() * r.float()),
                Div => {
                    if r.float() == 0. { return Err(self.err("Attempted division by zero!")); }
                    EvalType::Float(l.float() / r.float())
                }
            }
        })
    }
}
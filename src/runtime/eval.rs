use std::fmt::{Display, Formatter};
use crate::parse::node::Node;
use crate::runtime::op_eval;
use crate::runtime::runtime::Runner;
use crate::util::types::Number;

impl Runner {
    pub fn eval(&self, node: &Node) -> EvalType {
        let result = match node {
            Node::Literal(n) => match n {
                Number::Int(n) => EvalType::Int(*n),
                Number::Float(n) => EvalType::Float(*n),
                Number::Thought => self.thought.eval_type().unwrap(),
            }
            Node::Char(n) => {
                match self.eval(n) {
                    EvalType::Int(n) => EvalType::Char(charify(n)),
                    EvalType::Float(n) => EvalType::Char(charify(n.floor() as i32)),
                    EvalType::Char(c) => EvalType::Char(c)
                }
            }
            Node::Eval(l, o, r) => {
                self.op_eval(l.clone(), *o, r.clone())
            }
            Node::Out(n) => {
                match self.eval(n) {
                    EvalType::Int(n) => self.stack.get(&self.eval(&Node::Literal(Number::Int(n))).to_num().unwrap().int().unwrap()).unwrap().eval_type().unwrap(),
                    EvalType::Float(n) => self.stack.get(&self.eval(&Node::Literal(Number::Int(n as i32))).to_num().unwrap().int().unwrap()).unwrap().eval_type().unwrap(),
                    EvalType::Char(c) => {
                        if let Some(val) = self.stack.get(&(c as i32)).and_then(|v| v.eval_type()) {
                            val
                        } else {
                            EvalType::Int(0)
                        }
                    }
                }
            }
            _ => EvalType::Int(0)
        };
        result
    }
}
fn charify(i: i32) -> char {
    std::char::from_u32(i.rem_euclid(0x10FFFF) as u32).unwrap()
}
#[derive(Debug)]
pub enum EvalType {
    Int(i32),
    Float(f32),
    Char(char),
}
impl EvalType {
    pub fn to_num(&self) -> Result<Number, ()> {
        match self {
            EvalType::Int(i) => Ok(Number::Int(*i)),
            EvalType::Float(i) => Ok(Number::Float(*i)),
            EvalType::Char(_) => Err(()),
        }
    }
}
impl Display for EvalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalType::Int(x) => write!(f, "{x}"),
            EvalType::Float(x) => write!(f, "{x}"),
            EvalType::Char(x) => write!(f, "{x}")
        }
    }
}
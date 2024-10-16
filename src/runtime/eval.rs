use std::fmt::{Display, Formatter};
use crate::parse::Node;
use crate::runtime::op_eval;
use crate::runtime::runtime::Runner;
use crate::util::types::Number;

impl Runner {
    pub fn eval(&mut self, node: &Node) -> Result<EvalType, String> {
        let result = match node {
            Node::Literal(n) => {
                match n {
                    Number::Int(n) => {
                        EvalType::Int(n.clone())
                    },
                    Number::Float(n) => {
                        EvalType::Float(n.clone())
                    },
                    Number::Thought => self.thought.eval_type().unwrap(),
                }
            }
            Node::Char(n) => {
                match self.eval(n)? {
                    EvalType::Int(n) => EvalType::Char(charify(n)),
                    EvalType::Float(n) => EvalType::Char(charify(n.floor() as i64)),
                    EvalType::Char(c) => EvalType::Char(c)
                }
            }
            Node::Eval(l, o, r) => {
                self.op_eval(l.clone(), *o, r.clone())?
            }
            Node::Out(n) => {
                let index = match self.eval(n)? {
                    EvalType::Int(n) => n,
                    EvalType::Float(n) => n as i64,
                    EvalType::Char(c) => c as i64,
                };

                self.stack.get(&index).unwrap_or(&Number::Int(0)).eval_type().unwrap()
            }
            _ => EvalType::Int(0)
        };
        Ok(result)
    }
}
fn charify(i: i64) -> char {
    std::char::from_u32(i.rem_euclid(0x10FFFF) as u32).unwrap()
}
#[derive(Debug)]
pub enum EvalType {
    Int(i64),
    Float(f64),
    Char(char),
}
impl EvalType {
    pub fn to_num(&self) -> Result<Number, ()> {
        match self {
            EvalType::Int(i) => Ok(Number::Int(*i)),
            EvalType::Float(i) => Ok(Number::Float(*i)),
            EvalType::Char(_) => { Err(()) },
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
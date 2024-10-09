use std::collections::HashMap;
use std::ops::Deref;
use crate::parse::node::Node;
use crate::util::types::Number;

pub struct Runner {
    pub(crate) stack: HashMap<i64, Number>,
    pub(crate) thought: Number,
    expr: usize,
}
impl Runner {
    pub fn new() -> Self {
        Self {
            stack: HashMap::new(),
            thought: Number::Int(0),
            expr: 0,
        }
    }
    pub fn run(&mut self, nodes: Vec<Node>) -> Result<(), String> {
        for node in nodes {
            self.exec(node)?;
        }
        Ok(())
    }
    fn exec(&mut self, node: Node) -> Result<bool, String> {
        match node {
            Node::Print(d) => {
                print!("{}", self.eval(d.deref()));
                return Ok(false);
            }
            Node::Think(d) => {
                self.thought = self.eval(d.deref()).to_num()
                    .map_err(|_| format!("Unable to evaluate! Found at expression #{}", self.expr))?;
                return Ok(false);
            }
            Node::Assign(i, val) => {
                let index = i.to_num()?.int().ok_or_else(|| "Failed to convert index to integer")?;
                let value = match val.deref() {
                    Node::Out(n) => self.stack_get(n.to_num()?),
                    Node::Literal(n) => match *n {
                        Number::Int(m) => Number::Int(m),
                        Number::Float(m) => Number::Float(m),
                        Number::Thought => self.thought
                    },
                    Node::Eval(l, o, r) => self.eval(&Node::Eval(l.clone(), o.clone(), r.clone())).to_num().map_err(|()| String::new())?,
                    _ => return Err(format!("Unable to assign a non-data type to a stack index! Found at expression #{}", self.expr))
                };
                self.stack.insert(index, value);
                return Ok(false);
            }
            Node::If(cond, exec) => {
                if cond.to_num()? == self.thought {
                    for node in exec.deref() {
                        if self.exec(node.clone())? {
                            return Ok(true);
                        }
                    }
                }
                return Ok(false);
            }
            Node::Loop(exec) => {
                let mut broken = false;
                loop {
                    for node in exec.deref() {
                        if self.exec(node.clone())? {
                            broken = true;
                            break;
                        }
                    }
                    if broken { break }
                }
                return Ok(false);
            }
            Node::Break => {
                return Ok(true)
            }
            _ => {}
        }
        Ok(false)
    }
    fn stack_get(&self, i: Number) -> Number {
        let index = if i.int().is_none() {
            i.float().unwrap().floor() as i64
        } else {
            i.int().unwrap()
        };

        self.stack.get(&index).or(Some(&Number::Int(0))).unwrap().clone()
    }
}
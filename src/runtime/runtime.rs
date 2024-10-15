use std::collections::HashMap;
use std::ops::Deref;
use crate::parse::Node;
use crate::util::types::Number;

pub struct Runner {
    pub stack: HashMap<i64, Number>,
    pub thought: Number,
    pub macros: HashMap<String, Box<Vec<Node>>>,
    pub expr: usize,
}
impl Runner {
    pub fn new() -> Self {
        Self {
            stack: HashMap::new(),
            thought: Number::Int(0),
            macros: HashMap::new(),
            expr: 0,
        }
    }
    pub fn run(&mut self, nodes: Vec<Node>, macros: Option<HashMap<String, Box<Vec<Node>>>>) -> Result<(), String> {
        self.macros = macros.unwrap_or(HashMap::new());
        for node in nodes {
            self.exec(node)?;
        }
        Ok(())
    }
    fn exec(&mut self, node: Node) -> Result<bool, String> {
        match node {
            Node::Print(d) => {
                print!("{}", self.eval(d.deref())?);
                return Ok(false);
            }
            Node::Think(d) => {
                self.thought = self.eval(d.deref())?.to_num()
                    .map_err(|_| self.err("Unable to evaluate!"))?;
                return Ok(false);
            }
            Node::Assign(i, val) => {
                let index = match i.to_num()? {
                    Number::Int(n) => n,
                    Number::Float(n) => n.floor() as i64,
                    Number::Thought => self.thought.float().floor() as i64,
                };
                
                let value = match val.deref() {
                    Node::Out(n) => self.stack_get(n.to_num()?),
                    Node::Literal(n) => match *n {
                        Number::Int(m) => Number::Int(m),
                        Number::Float(m) => Number::Float(m),
                        Number::Thought => self.thought
                    },
                    Node::Eval(l, o, r) => self.eval(&Node::Eval(l.clone(), o.clone(), r.clone()))?.to_num().map_err(|()| String::new())?,
                    _ => return Err(self.err("Unable to assign a non-data type to a stack index!"))
                };
                self.stack.insert(index, value);
                return Ok(false);
            }
            Node::Run(s) => {
                let m = self.macros.get(&s).cloned().ok_or(self.err("Macro '{}' not found!"))?;
                for node in m.deref() {
                    if self.exec(node.clone())? {
                        return Ok(true);
                    }
                }
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
            i.float().floor() as i64
        } else {
            i.int().unwrap()
        };

        self.stack.get(&index).or(Some(&Number::Int(0))).unwrap().clone()
    }
}
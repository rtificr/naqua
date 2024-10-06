use std::panic::resume_unwind;
use crate::parse::node::Node;
use crate::parse::parser::Parser;
use crate::tokenize::token::Token;
use crate::util::types::{Keyword, Number};
use crate::util::types::Number::Thought;

impl<'t> Parser<'t> {
    pub fn parse_num(&mut self, num: Number) -> Result<Option<Node>, String> {
        println!("Parsing number {num:?}...");
        self.advance();

        let mut node = if num.thought() { Node::Literal(Number::Thought) } else { Node::Literal(num) };

        if let Some(Token::OpToken(o)) = self.peek().cloned() {
            self.advance(); // Move past the operator
            let next = match self.peek().cloned() {
                Some(Token::Data(n)) => {
                    // print!("1 - ");
                    // self.advance(); // Move past the number

                    self.parse_num(n.clone())?.ok_or_else(|| format!("Unable to parse a number after an operator! Found at expression #{}", self.expr))?
                }
                _ => return Err(format!("Unable to find a number after an operator! Found at expression #{}", self.expr))
            };
            node = Node::Eval(
                Box::new(if num.thought() { Node::Literal(Number::Thought) } else { Node::Literal(num) }),
                o.clone(),
                Box::new(next),
            );
        }

        println!("Checking for char...");

        let result = match self.peek() {
            Some(&Token::Keyword(Keyword::Char)) => {
                println!("Char found!");
                self.advance();
                Ok(Some(Node::Char(Box::new(node))))
            }
            Some(_) | None => {
                println!("No char found!");
                Ok(Some(node))
            }
        };
        println!("Done parsing number, moving on...");;
        result
    }
    pub fn parse_num_head(&mut self, num: Number) -> Result<Option<Node>, String> {
        println!("Parsing number head {num:?}...");
        
        if num.int().is_none() {
            return Err(format!("Floats are not valid indices! Found at expression #{}", self.expr));
        }
        let result = self.parse_num(num)?.unwrap();
        let expr = self.expr.clone();

        println!("Head Peek: {:?}", self.peek());
        let next = match self.peek() {
            Some(t) => t.clone(),
            None => return Err(format!("Unable to process a lone number '{:?}'! Found at expression #{expr}", self.peek()))
        };
        println!("NEXT: {next:?}");
        match next {
            Token::Keyword(Keyword::In) => {
                self.advance(); // Move past the 'in' keyword
                if let Some(Token::Data(n)) = self.peek().cloned() {
                    if n.thought() { return Err(format!("The thought is not allowed be stored on the stack! Found at expression #{expr}")); }
                    self.advance(); // Move past the number
                    match self.parse_num(n) {
                        Ok(Some(t)) => Ok(Some(Node::Assign(Box::new(result), Box::new(t)))),
                        Ok(None) => Err(format!("Unable to assign stack index to nothing! Found at expression #{expr}")),
                        Err(e) => Err(e)
                    }
                } else { Err(format!("Unable to assign stack index to a non-data type! Found at expression #{expr}")) }
            }
            Token::Keyword(Keyword::Out) => {
                self.advance(); // Move past the 'out' keyword
                match self.parse_out() {
                    Ok(Some(m)) => Ok(Some(Node::Out(Box::new(m)))),
                    Ok(None) => Err(format!("Unable to retrieve from a non-existent stack index! Found at expression #{expr}")),
                    Err(e) => Err(e)
                }
            }
            Token::Data(Thought) => {
                self.advance(); // Move past the 'thought' keyword
                Ok(Some(Node::Literal(Thought)))
            }
            Token::Data(_) => {
                self.advance(); // Move past the number
                Err(format!("Token '{next:?}' is not allowed to follow a heading number! Found at expression #{expr}"))
            }
            _ => Err(format!("Token '{next:?}' is not allowed to follow a heading number! Found at expression #{expr}"))
        }
    }
    pub fn parse_out(&mut self) -> Result<Option<Node>, String> {
        self.advance();

        println!("Parsing Out...");

        let expr = self.expr.clone();

        match self.peek() {
            Some(Token::Data(n)) => {
                match self.parse_num(*n) {
                    Ok(Some(m)) => Ok(Some(Node::Out(Box::new(m)))),
                    Ok(None) => Err(format!("Unable to retrieve from a non-existent stack index! Found at expression #{expr}")),
                    Err(e) => Err(e)
                }
            }
            Some(Token::Keyword(Keyword::Out)) => {
                match self.parse_out() {
                    Ok(Some(m)) => Ok(Some(Node::Out(Box::new(m)))),
                    Ok(None) => Err(format!("Unable to retrieve from a non-existent stack index! Found at expression #{expr}")),
                    Err(e) => Err(e)
                }
            }
            Some(_) => {
                Err(format!("Unable to get a non-integer index! Found at expression #{}", self.expr))
            }
            None => Err(format!("Unable to retrieve data from a non-existent index! Found at expression #{}", self.expr))
        }
    }
}
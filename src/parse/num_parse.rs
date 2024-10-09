use std::panic::resume_unwind;
use crate::parse::node::Node;
use crate::parse::parser::Parser;
use crate::tokenize::token::Token;
use crate::util::err::err_code;
use crate::util::types::{Keyword, Number};
use crate::util::types::Number::Thought;

impl<'t> Parser<'t> {
    pub fn parse_num(&mut self, token: Token) -> Result<Option<Node>, String> {
        let mut node = match token {
            Token::Data(num) => {
                let r = if num.thought() { Node::Literal(Thought) } else { Node::Literal(num) };
                self.advance();
                r
            },
            Token::Keyword(Keyword::Out) => {
                let r = self.parse_out()?.ok_or_else(|| format!("Unable to parse an out token! Found at expression #{}", self.expr))?;
                self.advance();
                r
            }
            _ => return Err(format!("Unable to parse a non-number token! Found at expression #{}", self.expr))
        };

        match self.peek().cloned() {
            Some(Token::NewLine) => {
                if self.log { println!("Newline found, continuing!"); }
                self.undo();
                return Ok(Some(node));
            }
            Some(Token::OpToken(o)) => {
                self.advance(); // Move past the operator
                let next = match self.peek().cloned() {
                    Some(Token::Data(n)) => {
                        self.parse_num(Token::Data(n))?.ok_or_else(|| format!("Unable to parse a number after an operator! Found at expression #{}", self.expr))?
                    }
                    Some(Token::Keyword(Keyword::Out)) => {
                        self.parse_num(Token::Keyword(Keyword::Out))?.ok_or_else(|| format!("Unable to parse a number after an operator! Found at expression #{}", self.expr))?
                    }
                    _ => return Err(format!("Unable to find a number after an operator! Found at expression #{}", self.expr))
                };
                node = Node::Eval(
                    Box::new(node),
                    o.clone(),
                    Box::new(next),
                );
            }
            _ => {
                self.undo();
                if self.log { println!("No operator found!") }
            },
        }

        if self.log { println!("Checking for char..."); }

        let result = match self.advance() {
            Some(Token::NewLine) => {
                Ok(Some(node))
            }
            Some(&Token::Keyword(Keyword::Char)) => {
                if self.log { println!("Char found!"); }
                Ok(Some(Node::Char(Box::new(node))))
            }
            Some(_) | None => {
                if self.log { println!("No char found!"); }
                self.undo();
                Ok(Some(node))
            }
        };

        if self.log { println!("Done parsing number {token:?}"); }
        result
    }

    pub fn parse_num_head(&mut self) -> Result<Option<Node>, String> {
        let num = match self.peek().unwrap().clone() {
            Token::Data(n) => n,
            _ => return Err(err_code(102))
        };
        if self.log { println!("Parsing number head {num:?}..."); }

        if num.int().is_none() {
            return Err(format!("Floats are not valid indices! Found at expression #{}", self.expr));
        }
        if self.log { println!("Parsing number within head {num:?}..."); }
        let result = self.parse_num(Token::Data(num))?.unwrap();
        let expr = self.expr.clone();

        if self.log { println!("Head Peek: {:?}", self.peek()); }
        let next = match self.advance() {
            Some(t) => t.clone(),
            None => return Err(format!("Unable to process a lone number '{:?}'! Found at expression #{expr}", self.peek()))
        };
        if self.log { println!("NEXT: {next:?}"); }

        let result = match next {
            Token::NewLine => {
                self.advance();
                Ok(None)
            }
            Token::Keyword(Keyword::In) => {
                if self.log { println!("In found!"); }
                self.advance(); // Move past the 'in' keyword
                match self.peek().clone() {
                    Some(n) => match self.parse_num(*n) {
                        Ok(Some(t)) => Ok(Some(Node::Assign(Box::new(result), Box::new(t)))),
                        Ok(None) => Err(format!("Unable to assign stack index to nothing! Found at expression #{expr}")),
                        Err(e) => Err(e)
                    },
                    None => Err(format!("Unable to assign stack index to nothing! Found at expression #{expr}"))
                }
            }
            Token::Keyword(Keyword::Out) => {
                match self.parse_out() {
                    Ok(Some(m)) => Ok(Some(Node::Out(Box::new(m)))),
                    Ok(None) => Err(format!("Unable to retrieve from a non-existent stack index! Found at expression #{expr}")),
                    Err(e) => Err(e)
                }
            }
            Token::Data(_) => {
                self.advance(); // Move past the number
                Err(format!("Token '{next:?}' is not allowed to follow a heading number! Found at expression #{expr}"))
            }
            _ => Err(format!("Token '{next:?}' is not allowed to follow a heading number! Found at expression #{expr}"))
        };

        self.advance();
        if self.log { println!("Done parsing number head {num:?}"); }
        result
    }

    pub fn parse_out(&mut self) -> Result<Option<Node>, String> {
        if self.log { println!("Parsing Out..."); }
        self.advance();

        let result = match self.peek().cloned() {
            Some(Token::NewLine) => {
                self.advance();
                Ok(None)
            }
            Some(Token::Data(n)) => {
                match n {
                    Number::Int(_) => {
                        let r = Node::Out(Box::new(Node::Literal(n)));
                        Ok(Some(r))
                    }
                    _ => Err(format!("Only integers can be used as stack indices! Found at expression #{}", self.expr))
                }
            }
            Some(_) => {
                Err(format!("Unable to get a non-integer index! Found at expression #{}", self.expr))
            }
            None => Err(format!("Unable to retrieve data from a non-existent index! Found at expression #{}", self.expr))
        };

        if self.log { println!("Done parsing Out!"); }

        result
    }
}
use crate::parse::Node;
use crate::parse::parser::Parser;
use crate::tokenize::token::Token;
use crate::util::types::Keyword;

impl<'t> Parser<'t> {
    pub fn parse_think(&mut self) -> Result<Option<Node>, String> {
        self.advance();
        let expr = self.expr.clone();

        let result = match self.peek() {
            Some(Token::NewLine) => {
                self.advance();
                Ok(None)
            }
            Some(Token::Data(n)) => {
                match self.parse_num(0, Token::Data(n.clone()))? {
                    Some(r) => Ok(Some(Node::Think(Box::new(r)))),
                    None => Ok(None)
                }
            }
            Some(Token::Keyword(Keyword::Out)) => {
                if self.log { println!("Parsing Think Out..."); }
                match self.parse_num(0, Token::Keyword(Keyword::Out))? {
                    Some(m) => Ok(Some(Node::Think(Box::new(m)))),
                    None => Err(self.err("Unable to retrieve from a non-existent stack index!"))
                }
            }
            Some(_) => Err(self.err("Only data types can be imagined!")),
            None => Err(self.err("Unable to imagine nothing!"))
        };
        self.advance();
        result
    }
    pub fn parse_print(&mut self) -> Result<Option<Node>, String> {
        if self.log { println!("Parsing Print..."); }
        self.advance();

        let result = match self.peek() {
            Some(Token::NewLine) => {
                self.advance();
                Ok(None)
            }
            Some(Token::Data(n)) => {
                if self.log { println!("Parsing Print Data..."); }
                match self.parse_num(0, Token::Data(n.clone()))? {
                    Some(r) => Ok(Some(Node::Print(Box::new(r)))),
                    None => Ok(None)
                }
            }
            Some(Token::Keyword(Keyword::Out)) => {
                if self.log { println!("Parsing Print Out..."); }
                match self.parse_num(0, Token::Keyword(Keyword::Out))? {
                    Some(m) => Ok(Some(Node::Print(Box::new(m)))),
                    None => Err(self.err("Unable to retrieve from a non-existent stack index!"))
                }
            }
            Some(_) => Err(self.err("Only data types can be printed!")),
            None => Err(self.err("Unable to print nothing!"))
        };
        self.advance();
        result
    }
    pub fn parse_run(&mut self) -> Result<Option<Node>, String> {
        if self.log { println!("Parsing Run..."); }
        self.advance();

        let result = match self.peek() {
            Some(Token::NewLine) => {
                self.advance();
                Ok(None)
            }
            Some(Token::RTKeyword(s)) => {
                Ok(Some(Node::Run(s.clone())))
            }
            Some(Token::Keyword(_)) => Err(self.err("Unable to run a macro with reserved name!")),
            Some(Token::Data(_)) => Err(self.err("Unable to run a data type!")),
            _ => Err(self.err("Unable to run a non-existent macro!"))
        };
        self.advance();
        result
    }
    pub fn parse_spawn(&mut self) -> Result<Option<Node>, String> {
        if self.log { println!("Parsing Run..."); }
        self.advance();

        let result = match self.peek() {
            Some(Token::NewLine) => {
                self.advance();
                Ok(None)
            }
            Some(Token::RTKeyword(s)) => {
                Ok(Some(Node::Spawn(s.clone())))
            }
            Some(Token::Keyword(_)) => Err(self.err("Unable to run a macro with reserved name!")),
            Some(Token::Data(_)) => Err(self.err("Unable to run a data type!")),
            _ => Err(self.err("Unable to run a non-existent macro!"))
        };
        self.advance();
        result
    }
}
use std::thread::sleep;
use crate::parse::node::Node;
use crate::tokenize::token::Token;
use crate::util::types::{Keyword};

pub struct Parser<'t> {
    tokens: &'t Vec<Token>,
    pub pos: usize,
    pub expr: usize,
}

impl<'t> Parser<'t> {
    pub fn new(tokens: &'t Vec<Token>) -> Self {
        Self { tokens, pos: 0, expr: 0 }
    }
    pub fn parse(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        println!("Parsing...");
        loop {
            match self.parse_expression() {
                Ok(Some(n)) => {
                    nodes.push(n);
                    self.expr += 1;
                }
                Ok(None) => break,
                Err(e) => return Err(format!("Parsing error: {e}"))
            }
        }
        println!("Parsed!");

        Ok(nodes)
    }
    pub fn parse_expression(&mut self) -> Result<Option<Node>, String> {
        let expr = self.expr.clone();

        let token = match self.peek().clone() {
            Some(token) => token.clone(),
            None => return Ok(None)
        };
        println!("Parsing expression #{expr}, starting with token '{token:?}'...");
        let r = match token {
            Token::Keyword(k) => {
                match k {
                    Keyword::Think => self.parse_think(),
                    Keyword::In => Err(format!("'in' must follow a stack index! Found at expression #{expr}")),
                    Keyword::Out => self.parse_out(),
                    Keyword::Print => self.parse_print(),
                    Keyword::If => self.parse_if(),
                    Keyword::Loop => self.parse_loop(),
                    Keyword::Break => {
                        self.advance();
                        Ok(Some(Node::Break))
                    },
                    _ => {
                        println!("Keyword not recognized");
                        self.advance();
                        Ok(Some(Node::Break))
                    }
                }
            }
            Token::Data(n) => self.parse_num_head(n),
            Token::OpToken(o) => Err(format!("Unexpected operator '{o}' at expression {expr}")),
            _ => return Ok(None)
        };
        
        r
    }
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    pub fn advance(&mut self) -> Option<&Token> {
        self.pos += 1;
        let p = self.peek();
        println!("Position {} > {p:?}", self.pos);
        p
    }
    pub fn undo(&mut self) -> Option<&Token> {
        self.pos -= 1;
        println!("Undoing...");
        self.peek()
    }
}
use std::fmt::format;
use std::thread::sleep;
use colored::Colorize;
use crate::parse::node::Node;
use crate::tokenize::token::Token;
use crate::util::types::{Keyword, Number};

pub struct Parser<'t> {
    tokens: &'t Vec<Token>,
    pub pos: usize,
    pub expr: usize,
    pub log: bool,
}

impl<'t> Parser<'t> {
    pub fn new(tokens: &'t Vec<Token>, log: bool) -> Self {
        Self { tokens, pos: 0, expr: 0, log }
    }
    pub fn parse(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        //println!("Parsing...");
        loop {
            match self.parse_expression() {
                Ok(Some(n)) => {
                    nodes.push(n);
                    self.expr += 1;
                }
                Ok(None) => break,
                Err(e) => return Err(e)
            }
        }
        //println!("Parsed!");

        Ok(nodes)
    }
    pub fn parse_expression(&mut self) -> Result<Option<Node>, String> {
        let expr = self.expr.clone();
        self.skip_newlines();   

        let token = match self.peek().clone() {
            Some(token) => token.clone(),
            None => return Ok(None)
        };


        if self.log {
            eprintln!("{}", format!("Parsing expression #{expr}, starting with token '{token:?}'...").bright_blue());
        }
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
                    }
                    _ => {
                        eprintln!("Keyword not recognized! Found at expression #{expr}");
                        self.advance();
                        Ok(Some(Node::Break))
                    }
                }
            }
            Token::Data(_) => self.parse_num_head(),
            Token::OpToken(o) => Err(format!("Unexpected operator '{o}' at expression {expr}")),
            _ => return Ok(None)
        };

        r
    }
    fn skip_newlines(&mut self) {
        while let Some(Token::NewLine) = self.peek() {
            self.advance();
        }
    }
    pub fn peek(&self) -> Option<&Token> {
        self.display();
        self.tokens.get(self.pos)
    }
    pub fn advance(&mut self) -> Option<&Token> {
        self.pos += 1;
        let p = self.peek();
        if self.log {
            self.display();
        }
        p
    }
    pub fn undo(&mut self) -> Option<&Token> {
        self.pos -= 1;
        if self.log {
            println!("Undoing...");
            self.display();
        }
        self.peek()
    }
    fn display(&self) {
        let toks = self.tokens.clone();
        print!("{:<10}", format!("POS {} > ", self.pos));
        for (i, val) in toks.iter().enumerate() { 
            let txt = match val {
                Token::Data(d) => match d {
                    Number::Int(i) => i.to_string(),
                    Number::Float(i) => i.to_string(),
                    Number::Thought => "Thought".to_string(),
                }
                Token::OpToken(o) => o.to_char().to_string(),
                Token::Keyword(k) => k.to_str().to_string(),
                _ => format!("{:?}", val)
            };

            if i == self.pos {
                print!("{}", format!("{txt} ").red());
            } else {
                print!("{txt} ");
            }
        }
        println!();
    }
}
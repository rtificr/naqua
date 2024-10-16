use std::collections::HashMap;
use std::fmt::format;
use std::thread::sleep;
use colored::Colorize;
use crate::parse::{ExprType, Node, ParserResult};
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
    pub fn parse(&mut self) -> Result<ParserResult, String> {
        let mut nodes = Vec::new();
        let mut macros = Vec::new();
        //println!("Parsing...");
        loop {
            match self.parse_expression() {
                Ok(Some(n)) => {
                    if self.log { println!("{:?}", n); }
                    match n {
                        ExprType::Node(Some(node)) => nodes.push(node),
                        ExprType::Macro(node) => macros.push(node),
                        _ => {}
                    }
                    self.expr += 1;
                }
                Ok(None) => break,
                Err(e) => return Err(e)
            }
        }
        //println!("Parsed!");
        let mut macro_map = HashMap::new();
        for (name, body) in macros {
            macro_map.insert(name, body);
        };
        
        Ok(ParserResult {
            nodes,
            macros: if !macro_map.is_empty() {
                Some(macro_map)
            } else {
                None
            }
        })
    }
    pub fn parse_expression(&mut self) -> Result<Option<ExprType>, String> {
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
                    Keyword::Think => ExprType::Node(self.parse_think()?),
                    Keyword::In => return Err(self.err("'in' must follow a stack index!")),
                    Keyword::Out => ExprType::Node(self.parse_out()?),
                    Keyword::Print => ExprType::Node(self.parse_print()?),
                    Keyword::If => ExprType::Node(self.parse_if()?),
                    Keyword::Loop => ExprType::Node(self.parse_loop()?),
                    Keyword::Run => ExprType::Node(self.parse_run()?),
                    Keyword::Define => {
                        ExprType::Macro(self.parse_def()?)
                    },
                    Keyword::Break => {
                        self.advance();
                        ExprType::Node(Some(Node::Break))
                    },
                    Keyword::Spawn => ExprType::Node(self.parse_spawn()?),
                    _ => {
                        return Err(self.err("Keyword not recognized!"));
                    }
                }
            }
            Token::Data(_) => ExprType::Node(self.parse_num_head()?),
            Token::OpToken(o) => return Err(self.err(format!("Unexpected operator '{}' found!", o.to_char()).as_str())),
            _ => return Ok(None)
        };

        Ok(Some(r))
    }
    fn skip_newlines(&mut self) {
        while let Some(Token::NewLine) = self.peek() {
            self.advance();
        }
    }
    pub fn peek(&self) -> Option<&Token> {
        if self.log {
            self.display();
        }
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
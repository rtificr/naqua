use crate::parse::{ExprType, Node};
use crate::parse::parser::{Parser};
use crate::tokenize::token::Token;
use crate::util::err::err_code;

impl<'t> Parser<'t> {
    pub fn parse_if(&mut self) -> Result<Option<Node>, String> {
        self.advance();

        let cond = match self.peek() {
            Some(c) => match self.parse_num(0, match c {
                Token::Data(n) => Token::Data(*n),
                _ => return Err(self.err("Expected a number for if statement condition"))
            }) {
                Ok(Some(n)) => n,
                Ok(None) => return Err(err_code(101)),
                Err(e) => return Err(e)
            },
            None => {
                self.undo();
                return Err(self.err("Expected a number for if statement condition"));
            }
        };

        let mut body = Vec::new();

        self.advance();

        if let Some(Token::OpenBrace) = self.peek() {
            let mut brace_count = 1;
            self.advance();

            while let Some(tok) = self.peek().cloned() {
                match tok {
                    Token::OpenBrace => {
                        brace_count += 1;
                        self.advance();
                    },
                    Token::CloseBrace => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            self.advance();
                            break;
                        }
                    },
                    Token::NewLine => {
                        self.advance();
                    },
                    _ => {
                        match self.parse_expression() {
                            Ok(Some(node)) => match node {
                                ExprType::Node(n) => if let Some(m) = n { body.push(m) },
                                ExprType::Macro(_) => return Err("Macro definitions are not allowed in if statements!".to_string()),
                            },
                            Ok(None) => {}, // Continue to next token
                            Err(e) => return Err(e), // Propagate the error
                        }
                    }
                }
            }
            if brace_count != 0 {
                return Err(self.err("Mismatched braces in if statement"));
            }
        } else {
            return Err(self.err("Expected opening brace for if statement"));
        }

        Ok(Some(Node::If(Box::new(cond), Box::new(body))))
    }
    pub fn parse_loop(&mut self) -> Result<Option<Node>, String> {
        self.advance();

        let mut body = Vec::new();

        if let Some(Token::OpenBrace) = self.peek() {
            let mut brace_count = 1;
            self.advance();

            while let Some(tok) = self.peek() {
                match tok {
                    Token::OpenBrace => {
                        brace_count += 1;
                        self.advance();
                    },
                    Token::CloseBrace => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            self.advance();
                            break;
                        }
                    },
                    _ => {
                        match self.parse_expression() {
                            Ok(Some(node)) => match node {
                                ExprType::Node(n) => if let Some(m) = n { body.push(m) },
                                ExprType::Macro(_) => return Err(self.err("Macro definitions are not allowed in loop statements!")),
                            },
                            Ok(None) => {}, // Continue to next token
                            Err(e) => return Err(e), // Propagate the error
                        }
                    }
                }
            }
            if brace_count != 0 {
                return Err(self.err("Mismatched braces in loop statement"));
            }
        } else {
            return Err(self.err("Expected opening brace for loop statement"));
        }

        Ok(Some(Node::Loop(Box::new(body))))
    }
    pub fn parse_def(&mut self) -> Result<(String, Box<Vec<Node>>), String> {
        self.advance();

        let name = match self.peek().cloned() {
            Some(Token::RTKeyword(c)) => {
                c
            }
            Some(_) | None => {
                self.undo();
                return Err(self.err("'define' statement needs a name!"));
            }
        };

        let mut body = Vec::new();

        self.advance();

        if let Some(Token::OpenBrace) = self.peek() {
            let mut brace_count = 1;
            self.advance();

            while let Some(tok) = self.peek().cloned() {
                match tok {
                    Token::OpenBrace => {
                        brace_count += 1;
                        self.advance();
                    },
                    Token::CloseBrace => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            self.advance();
                            break;
                        }
                    },
                    Token::NewLine => {
                        self.advance();
                    },
                    _ => {
                        match self.parse_expression() {
                            Ok(Some(node)) => match node {
                                ExprType::Node(n) => if let Some(m) = n { body.push(m) },
                                ExprType::Macro(_) => return Err(self.err("Macro definitions are not allowed to be in other macro definitions!")),
                            },
                            Ok(None) => {}, // Continue to next token
                            Err(e) => return Err(e), // Propagate the error
                        }
                    }
                }
            }
            if brace_count != 0 {
                return Err(self.err("Mismatched braces in macro definition at expression #{}"));
            }
        } else {
            return Err(self.err("Expected opening brace for macro definition at expression #{}"));
        }

        Ok((name, Box::new(body)))
    }
}
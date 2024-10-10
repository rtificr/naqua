use crate::parse::node::Node;
use crate::parse::parser::Parser;
use crate::tokenize::token::Token;
use crate::util::err::err_code;

impl<'t> Parser<'t> {
    pub fn parse_if(&mut self) -> Result<Option<Node>, String> {
        self.advance();
        
        let cond = match self.peek() {
            Some(c) => match self.parse_num(0, match c {
                Token::Data(n) => Token::Data(*n),
                _ => return Err(format!("Only data types can be compared to thoughts! Found at expression #{}", self.expr))
            }) {
                Ok(Some(n)) => n,
                Ok(None) => return Err(err_code(101)),
                Err(e) => return Err(e)
            },
            None => {
                self.undo();
                return Err(format!("'if' statement needs a condition! Found at expression #{}", self.expr));
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
                            Ok(Some(node)) => body.push(node),
                            Ok(None) => {}, // Continue to next token
                            Err(e) => return Err(e), // Propagate the error
                        }
                    }
                }
            }
            if brace_count != 0 {
                return Err(format!("Mismatched braces in if statement at expression #{}", self.expr));
            }
        } else {
            return Err(format!("Expected opening brace for if statement at expression #{}", self.expr));
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
                            Ok(Some(node)) => body.push(node),
                            Ok(None) => {}, // Continue to next token
                            Err(e) => return Err(e), // Propagate the error
                        }
                    }
                }
            }
            if brace_count != 0 {
                return Err(format!("Mismatched braces in loop statement at expression #{}", self.expr));
            }
        } else {
            return Err(format!("Expected opening brace for loop statement at expression #{}", self.expr));
        }

        Ok(Some(Node::Loop(Box::new(body))))
    }
}
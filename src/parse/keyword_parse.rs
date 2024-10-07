use crate::parse::node::Node;
use crate::parse::parser::Parser;
use crate::tokenize::token::Token;
use crate::util::err::err_code;
use crate::util::types::Keyword;

impl<'t> Parser<'t> {
    pub fn parse_think(&mut self) -> Result<Option<Node>, String>{
        self.advance();
        let expr = self.expr.clone();

        let result = match self.peek() {
            Some(Token::Data(n)) => {
                match self.parse_num(n.clone())? {
                    Some(r) => Ok(Some(Node::Think(Box::new(r)))),
                    None => Ok(None)
                }
            },
            Some(Token::Keyword(Keyword::Out)) => {
                match self.parse_out(){
                    Ok(Some(m)) => Ok(Some(Node::Think(Box::new(m)))),
                    Ok(None) => Err(format!("Unable to retrieve from a non-existent stack index! Found at expression #{expr}")),
                    Err(e) => Err(e)
                }
            }
            Some(_) => Err(format!("Only data types can be imagined!: Found at expression #{}", self.expr)),
            None => Err(format!("Unable to imagine nothing! Found at expression #{}", self.expr))
        };
        result
    }
    pub fn parse_print(&mut self) -> Result<Option<Node>, String> {
        self.advance();
        let expr = self.expr.clone();

        let result = match self.peek() {
            Some(Token::Data(n)) => {
                match self.parse_num(n.clone())? {
                    Some(r) => Ok(Some(Node::Print(Box::new(r)))),
                    None => Ok(None)
                }
            },
            Some(Token::Keyword(Keyword::Out)) => {
                match self.parse_out(){
                    Ok(Some(m)) => Ok(Some(Node::Think(Box::new(m)))),
                    Ok(None) => Err(format!("Unable to retrieve from a non-existent stack index! Found at expression #{expr}")),
                    Err(e) => Err(e)
                }
            }
            Some(_) => Err(format!("Only data types can be printed!: Found at expression #{}", self.expr)),
            None => Err(format!("Unable to print nothing! Found at expression #{}", self.expr))
        };
        result
    }
}
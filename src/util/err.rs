use std::string::ParseError;
use crate::parse::parser::Parser;
use crate::runtime::runtime::Runner;

pub fn err_code(code: i16) -> String {
    format!("Error code {code}")
}
impl Parser<'_> {
    pub fn err(&mut self, msg: &str) -> String {
        format!("{msg}: Found at expression #{}", self.expr)
    }
}
impl Runner {
    pub fn err(&mut self, msg: &str) -> String {
        format!("{msg}: Found at expression #{}", self.expr)
    }
}
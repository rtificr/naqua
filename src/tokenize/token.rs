use crate::util::types::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    Data(Number),
    Keyword(Keyword),
    OpToken(Operator)
}


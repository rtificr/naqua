use crate::util::types::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    Data(Number),
    Keyword(Keyword),
    OpToken(Operator),
    RTKeyword(String), // Run-time keyword, keyword evaluated at runtime (for macros)
    NewLine
}


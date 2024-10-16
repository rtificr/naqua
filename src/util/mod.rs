pub mod types;
pub mod err;

pub enum Is {}

impl Is {
    pub fn whitespace(c: char) -> bool { c.is_whitespace() }
    pub fn letter(c: char) -> bool { c.is_alphabetic() }
    pub fn symbol(c: char) -> bool { matches!(c, '_' | '-') }
    pub fn digit(c: char) -> bool { c.is_digit(10) }
    pub fn operator(c: char) -> bool { "+-*/^%".contains(c) }
    pub fn brace(c: char) -> bool { "{}".contains(c) }
    // Is a character allowed to be part of a runtime keyword's name?
    pub fn rtk_compatible(c: char) -> bool { Is::letter(c) || Is::digit(c) || Is::symbol(c) }
}
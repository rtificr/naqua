use crate::tokenize::token::Token;
use crate::tokenize::token::Token::OpToken;
use crate::util::Is;
use crate::util::types::{Keyword, Number, Operator};

pub struct Tokenizer {
    pos: usize,
    row: usize,
    input: String,
}
impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self { pos: 0, row: 0, input }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        println!("Tokenizing...");
        let mut tokens = Vec::new();
        let chars = self.input.chars().collect::<Vec<char>>();

        while self.pos < chars.len() {
            let c = chars[self.pos];

            if Is::whitespace(c) {
                if c == '\n' {
                    self.row += 1;
                }
                self.go();
                continue;
            }
            if Is::letter(c) {
                let start_pos = self.pos;
                while self.pos < chars.len() && Is::letter(chars[self.pos]) {
                    self.go();
                }
                let word = &self.input[start_pos..self.pos];
                let k = Keyword::from(word).ok_or(format!("'{word}' not a keyword"))?;

                if k == Keyword::Thought { tokens.push(Token::Data(Number::Thought)) }
                else { tokens.push(Token::Keyword(k)) };
                continue;
            }
            if Is::digit(c) || c == '-' {
                let start_pos = self.pos;
                let mut has_decimal = false;
                self.go();

                while self.pos < chars.len() {
                    let ch = chars[self.pos];
                    if Is::digit(ch) {
                        self.go();
                    } else if ch == '.' && !has_decimal {
                        has_decimal = true;
                        self.go();
                    } else {
                        break;
                    }
                }

                let number_str = &self.input[start_pos..self.pos];
                
                if c == '-' && number_str.len() == 1 {
                    tokens.push(OpToken(Operator::Sub));
                    continue
                }

                if has_decimal {
                    tokens.push(Token::Data(Number::Float(number_str.parse().map_err(|_| format!("Incorrect float formatting '{number_str}' at line {}", self.row))?)))
                } else {
                    tokens.push(Token::Data(Number::Int(number_str.parse().map_err(|_| format!("Incorrect integer formatting '{number_str}' at line {}", self.row))?)))
                }
                continue;
            }
            if Is::brace(c) {
                match c {
                    '{' => tokens.push(Token::OpenBrace),
                    '}' => tokens.push(Token::CloseBrace),
                    _ => {}
                }
                self.go();
                continue;
            }
            if Is::operator(c) {
                tokens.push(OpToken(Operator::from(c)));
                self.go();
                continue;
            }
        }

        Ok(tokens)
    }
    fn go(&mut self) {
        self.pos += 1;
    }
}
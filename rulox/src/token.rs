use std::{any::Any, fmt::format};

use crate::TokenType;

pub struct Token {
    ttype: TokenType,
    lexeme: String,
    line: usize,
    literal: Box<dyn Any>,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, line: usize, literal: Box<dyn Any>) -> Token {
        Token {
            ttype: ttype,
            lexeme: lexeme,
            line: line,
            literal: literal,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {:?}", self.ttype, self.lexeme, self.literal)
    }
}

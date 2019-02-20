mod token;
pub use token::Token;

use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    pub source: Chars<'a>,
    position: usize,
    read_position: usize,
    ch: Option<char>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Lexer {
        let mut chars = source.chars();
        Lexer {
            source: chars,
            position: 0,
            read_position: 0,
            ch: None
        } 
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.pop_char() {
            None => None,
            Some('=') => Some(Token::ASSIGN),
            Some('+') => Some(Token::PLUS),
            Some(',') => Some(Token::COMMA),
            Some(';') => Some(Token::SEMICOLON),
            Some('(') => Some(Token::LPAREN),
            Some(')') => Some(Token::RPAREN),
            Some('{') => Some(Token::LBRACE),
            Some('}') => Some(Token::RBRACE),
            Some(string) => {
                let string = self.collect_str(string);
                match string.parse::<u8>().is_ok() {
                    true => Some(Token::INT(string)),
                    false => Some(Token::IDENT(string))
                }
            },
        }
    }

    pub fn pop_char(&mut self) -> Option<char> {
        match self.source.next() {
            Some(' ') => self.pop_char(),
            Some(value) => Some(value),
            None => None
        }
    }

    pub fn is_next_alphanumeric(&self) -> Option<bool> {
        let mut source = self.source.clone();
        match source.peekable().peek() {
            Some(value) => { 
                Some(value.is_ascii_alphanumeric())
            },
            None => None
        }
    }

    pub fn collect_str(&mut self, unit: char) -> String {
        let mut unit = unit.to_string();
        while let Some(true) = self.is_next_alphanumeric() {
            unit.push(self.pop_char().unwrap_or_default())
        }
        unit
    }
}
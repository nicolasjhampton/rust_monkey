mod token;
pub use token::Token;

use std::str::Chars;
use std::iter::Peekable;

fn is_number(string: &String) -> bool {
    string.parse::<u8>().is_ok()
}

pub struct Lexer<'a> {
    pub source: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Lexer {
        let mut chars = source.chars().peekable();
        Lexer {
            source: chars
        } 
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.pop_char() {
            None => None,
            Some('+') => Some(Token::PLUS),
            Some(',') => Some(Token::COMMA),
            Some(';') => Some(Token::SEMICOLON),
            Some('(') => Some(Token::LPAREN),
            Some(')') => Some(Token::RPAREN),
            Some('{') => Some(Token::LBRACE),
            Some('}') => Some(Token::RBRACE),
            Some('=') => {
                match self.is_next_char('=') {
                    true => {
                        self.pop_char();
                        Some(Token::EQ)
                    },
                    false => Some(Token::ASSIGN)
                }
            },
            Some('!') => {
                match self.is_next_char('=') {
                    true => {
                        self.pop_char();
                        Some(Token::NE)
                    },
                    false => Some(Token::NOT)
                }
            },
            Some(string) => self.num_ident_or_keyword(string) 
        }
    }

    pub fn num_ident_or_keyword(&mut self, character: char) -> Option<Token> {
        let FN = String::from("fn");
        let LET = String::from("let");
        match self.collect_str(character) {
            Some(ref string) if is_number(string) => Some(Token::INT(string.parse().unwrap())),
            Some(ref string) if string == &FN => Some(Token::FUNCTION),
            Some(ref string) if string == &LET => Some(Token::LET),
            Some(string) => Some(Token::IDENT(string)),
            _ => None
        }
    }


    pub fn pop_char(&mut self) -> Option<char> {
        match self.source.next() {
            Some(' ') => self.pop_char(),
            Some(value) => Some(value),
            None => None
        }
    }

    pub fn is_next_char(&mut self, comp: char) -> bool {
        if let Some(value) = self.source.peek() {
            value == &comp
        } else {
            false
        }
    }

    pub fn is_next_alphanumeric(&mut self) -> Option<bool> {
        match self.source.peek() {
            Some(value) => { 
                Some(value.is_ascii_alphanumeric())
            },
            None => None
        }
    }

    pub fn collect_str(&mut self, unit: char) -> Option<String> {
        let mut unit = unit.to_string();
        while let Some(true) = self.is_next_alphanumeric() {
            unit.push(self.pop_char().unwrap_or_default())
        }
        Some(unit)
    }
}
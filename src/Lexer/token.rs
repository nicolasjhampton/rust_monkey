// pub trait Token {
//     fn token_type(&self) -> String;
//     fn literal(&self) -> String;
// }

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(String),
    INT(String),
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET
}



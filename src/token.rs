use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType{
    //One char tokens
    OpenParen, CloseParen,
    OpenBrace, CloseBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    //Comparison Tokens
    Equal, EqualEqual,
    Bang, BangEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
       
    //Literals
    Identifier, String, Number,

    //Keywords
    If, Else, And, Or, True, False,
    For, While, Let, Fn, Class, Return,
    Nil, Print, Super, This,

    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LitType {
    String(String),
    Num(isize),
    Bool(bool),
    None
}

impl fmt::Display for LitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LitType::String(value) => write!(f, "{}", value),
            LitType::Num(value) => write!(f, "{}", value),
            LitType::Bool(value) => write!(f, "{}", value),
            LitType::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LitType,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String,
        literal: LitType, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.token_type, self.lexeme, self.literal)
    }
}


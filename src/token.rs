#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub enum Literal {
    StringLit(String),
    NumLit(f64),
    NotLit
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String,
        literal: Literal, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        "{self.token_type} {self.lexeme} {self.literal}".to_string()
    }
}


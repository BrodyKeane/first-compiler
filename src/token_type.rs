enum TokenType{
    //One char tokens
    OpenParen, CloseParen,
    OpenBrace, CloseBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    //Comparison Tokens
    Equal, Equal_Equal,
    Bang, Bang_Equal,
    Greater, Greater_Equal,
    Less, Less_Equal,
       
    //Literals
    Identifier, String, Number,

    //Keywords
    If, Else, And, Or, True, False,
    For, While, Let, Fn, Class, Return,
    Nil, Print, Super, This,

    EOF
}

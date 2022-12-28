use std::fmt::{
    write,
    Display,
    Formatter,
};

#[derive(Debug)]
pub enum Token {
    Keyword(SqlKeyword),
    String(String),
    Asterisk,
    Multiply,
    Divide,
    Plus,
    Minus,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    Neq,
    LParen,
    RParen,
    Semicolon,
    Comma,
    Identifier(String),
    EOF,
}

pub enum SqlKeyword {
    And,
    As,
    Begin,
    Commit,
    Delete,
    Drop,
    Exists,
    False,
    From,
    Full,
    If,
    Inner,
    Insert,
    Into,
    Join,
    Left,
    Not,
    On,
    Or,
    Outer,
    Right,
    Select,
    Set,
    Table,
    Transaction,
    True,
    Update,
    Where,
    Write,
}

/* impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Token::*;

        let str_representation = match *self {
            Select => "SELECT",
            From => "FROM",
            EndOfString => "\n",
        };
        write!(f, "{}", str_representation)
    }
} */

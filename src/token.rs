#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Ident(String),
    Number(i64),
    Plus,
    Minus,
    Equals, // =
    Semicolon,
    Star,
    Slash,
    RParen,
    LParen,
    EqEq,      // ==
    NotEq,     // !=
    Less,      // <
    LessEq,    // <=
    Greater,   // >
    GreaterEq, // >=
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    UnexpectedChar { ch: char, pos: usize },
    NumberTooLarge { pos: usize },
}

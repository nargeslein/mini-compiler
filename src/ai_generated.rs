//! Lexer for a tiny expression language.
//!
//! Tokens: `let`, identifiers, integer literals and
//! `+ - * / = ; ( ) == != < <= > >=`.
//! Whitespace separates tokens and is otherwise ignored.
//!
//! Run the tests with `cargo test`.

use crate::token::{LexError, Token};
use std::iter::Peekable;
use std::str::CharIndices;

/// Splits `input` into tokens.
///
/// # Errors
///
/// Returns an error if `input` contains a character that cannot start a
/// token, or an integer literal that does not fit in an `i64`. `pos` is the
/// byte offset where the offending token starts.
pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((start, c)) = chars.next() {
        let token = match c {
            c if c.is_whitespace() => continue,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '=' if followed_by_eq(&mut chars) => Token::EqEq,
            '=' => Token::Equals,
            '!' if followed_by_eq(&mut chars) => Token::NotEq,
            '<' if followed_by_eq(&mut chars) => Token::LessEq,
            '<' => Token::Less,
            '>' if followed_by_eq(&mut chars) => Token::GreaterEq,
            '>' => Token::Greater,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '0'..='9' => {
                let digits = lexeme(input, &mut chars, start, |c| c.is_ascii_digit());
                // `digits` is all ASCII digits, so the only possible failure is overflow.
                match digits.parse::<i64>() {
                    Ok(n) => Token::Number(n),
                    Err(_) => return Err(LexError::NumberTooLarge { pos: start }),
                }
            }
            c if is_ident_start(c) => match lexeme(input, &mut chars, start, is_ident_continue) {
                "let" => Token::Let,
                name => Token::Ident(name.to_string()),
            },
            _ => return Err(LexError::UnexpectedChar { ch: c, pos: start }),
        };
        tokens.push(token);
    }
    Ok(tokens)
}

/// Advances `chars` past every character accepted by `more`, then returns the
/// slice of `input` from byte offset `start` up to the new position.
fn lexeme<'a>(
    input: &'a str,
    chars: &mut Peekable<CharIndices<'a>>,
    start: usize,
    more: impl Fn(char) -> bool,
) -> &'a str {
    while chars.next_if(|&(_, c)| more(c)).is_some() {}
    let end = chars.peek().map_or(input.len(), |&(i, _)| i);
    &input[start..end]
}

/// Consumes the next character if it is `=`, for two-character operators
/// like `==` and `<=`.
fn followed_by_eq(chars: &mut Peekable<CharIndices>) -> bool {
    chars.next_if(|&(_, c)| c == '=').is_some()
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

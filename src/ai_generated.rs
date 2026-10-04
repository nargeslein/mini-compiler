//! Lexer for a tiny expression language.
//!
//! Tokens: `let`, identifiers, integer literals and `+ - * / = ; ( )`.
//! Whitespace separates tokens and is otherwise ignored.
//!
//! Run the tests with `cargo test`, or without Cargo:
//! `rustc --edition 2021 --test ai_generated.rs && ./ai_generated`

use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Ident(String),
    Number(i64),
    Plus,
    Minus,
    Equals,
    Semicolon,
    Star,
    Slash,
    LParen,
    RParen,
}

/// Splits `input` into tokens.
///
/// # Panics
///
/// The return type has no room for an error, so this panics if `input`
/// contains a character that cannot start a token, or an integer literal that
/// does not fit in an `i64`.
pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((start, c)) = chars.next() {
        let token = match c {
            c if c.is_whitespace() => continue,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '=' => Token::Equals,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '0'..='9' => {
                let digits = lexeme(input, &mut chars, start, |c| c.is_ascii_digit());
                // `digits` is all ASCII digits, so the only possible failure is overflow.
                match digits.parse::<i64>() {
                    Ok(n) => Token::Number(n),
                    Err(_) => {
                        panic!(
                            "number `{}` at byte {} does not fit in an i64",
                            digits, start
                        )
                    }
                }
            }
            c if is_ident_start(c) => match lexeme(input, &mut chars, start, is_ident_continue) {
                "let" => Token::Let,
                name => Token::Ident(name.to_string()),
            },
            _ => panic!("unexpected character {:?} at byte {}", c, start),
        };
        tokens.push(token);
    }
    tokens
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

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::Token::*;
    use super::*;

    fn ident(name: &str) -> Token {
        Ident(name.to_string())
    }

    // --- The requested inputs ---

    #[test]
    fn symbols() {
        assert_eq!(lex("= + ;"), vec![Equals, Plus, Semicolon]);
    }

    #[test]
    fn numbers() {
        assert_eq!(lex("42 + 7;"), vec![Number(42), Plus, Number(7), Semicolon]);
    }

    #[test]
    fn identifiers() {
        assert_eq!(
            lex("x + abc;"),
            vec![ident("x"), Plus, ident("abc"), Semicolon]
        );
    }

    #[test]
    fn let_statement() {
        assert_eq!(
            lex("let x = 42 + y;"),
            vec![
                Let,
                ident("x"),
                Equals,
                Number(42),
                Plus,
                ident("y"),
                Semicolon,
            ]
        );
    }

    #[test]
    fn identifier_containing_digit() {
        assert_eq!(
            lex("x1 + abc;"),
            vec![ident("x1"), Plus, ident("abc"), Semicolon]
        );
    }

    #[test]
    fn identifier_containing_underscore() {
        assert_eq!(lex("my_var;"), vec![ident("my_var"), Semicolon]);
    }

    #[test]
    fn parenthesised_arithmetic() {
        assert_eq!(
            lex("let z = (x - 1) * 2 / y;"),
            vec![
                Let,
                ident("z"),
                Equals,
                LParen,
                ident("x"),
                Minus,
                Number(1),
                RParen,
                Star,
                Number(2),
                Slash,
                ident("y"),
                Semicolon,
            ]
        );
    }

    // --- Edge cases and invalid input ---

    #[test]
    fn empty_and_whitespace_only_input() {
        assert_eq!(lex(""), Vec::<Token>::new());
        assert_eq!(lex(" \t\r\n"), Vec::<Token>::new());
    }

    #[test]
    fn whitespace_between_tokens_is_optional() {
        assert_eq!(
            lex("let\tx=(1+y);\n"),
            vec![
                Let,
                ident("x"),
                Equals,
                LParen,
                Number(1),
                Plus,
                ident("y"),
                RParen,
                Semicolon,
            ]
        );
    }

    #[test]
    fn keyword_only_matches_a_whole_identifier() {
        assert_eq!(
            lex("letter let_ _let let1 Let"),
            vec![
                ident("letter"),
                ident("let_"),
                ident("_let"),
                ident("let1"),
                ident("Let"),
            ]
        );
    }

    #[test]
    fn largest_i64_is_accepted() {
        assert_eq!(lex("9223372036854775807"), vec![Number(i64::MAX)]);
    }

    #[test]
    #[should_panic(expected = "number `9223372036854775808` at byte 0 does not fit in an i64")]
    fn number_too_large_for_i64_panics() {
        lex("9223372036854775808");
    }

    #[test]
    #[should_panic(expected = "unexpected character '$' at byte 6")]
    fn unexpected_character_panics() {
        lex("x = 4 $ 2;");
    }
}

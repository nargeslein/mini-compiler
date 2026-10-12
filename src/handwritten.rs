use crate::token::{LexError, Token};
use std::iter::Peekable;
use std::str::CharIndices;

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((pos, c)) = chars.next() {
        match c {
            '=' | '<' | '>' | '!' => tokens.push(read_operator(c, &mut chars, pos)?),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            ';' => tokens.push(Token::Semicolon),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '0'..='9' => tokens.push(Token::Number(read_number(c, &mut chars, pos)?)),
            c if c.is_alphabetic() || c == '_' => {
                tokens.push(match_token(read_word(c, &mut chars)))
            }
            c if c.is_whitespace() => {}
            _ => return Err(LexError::UnexpectedChar { ch: c, pos }),
        }
    }
    Ok(tokens)
}

fn read_number(
    first: char,
    chars: &mut Peekable<CharIndices>,
    start: usize,
) -> Result<i64, LexError> {
    let mut number = first.to_string();
    while let Some(&(_, c)) = chars.peek() {
        if !c.is_ascii_digit() {
            break;
        }
        number.push(c);
        chars.next();
    }
    number
        .parse()
        .map_err(|_| LexError::NumberTooLarge { pos: start })
}

fn read_word(first: char, chars: &mut Peekable<CharIndices>) -> String {
    let mut string = first.to_string();
    while let Some(&(_, c)) = chars.peek() {
        if !(c.is_alphanumeric() || c == '_') {
            break;
        }
        string.push(c);
        chars.next();
    }
    string
}

fn match_token(name: String) -> Token {
    match name.as_str() {
        "let" => Token::Let,
        _ => Token::Ident(name),
    }
}

fn read_operator(
    first: char,
    chars: &mut Peekable<CharIndices>,
    start: usize,
) -> Result<Token, LexError> {
    let followed_by_eq = chars.next_if(|&(_, c)| c == '=').is_some();

    match (first, followed_by_eq) {
        ('=', true) => Ok(Token::EqEq),
        ('=', false) => Ok(Token::Equals),
        ('<', true) => Ok(Token::LessEq),
        ('<', false) => Ok(Token::Less),
        ('>', true) => Ok(Token::GreaterEq),
        ('>', false) => Ok(Token::Greater),
        ('!', true) => Ok(Token::NotEq),
        ('!', false) => Err(LexError::UnexpectedChar {
            ch: first,
            pos: start,
        }),
        _ => unreachable!(),
    }
}

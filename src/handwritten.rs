use std::iter::Peekable;
use std::str::Chars;

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
    RParen,
    LParen,
}
#[derive(Debug, PartialEq)]
pub enum LexError {
    UnexpectedChar(char),
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '=' => tokens.push(Token::Equals),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            ';' => tokens.push(Token::Semicolon),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '0'..='9' => tokens.push(Token::Number(read_number(c, &mut chars))),
            c if c.is_alphabetic() || c == '_' => {
                tokens.push(match_token(read_word(c, &mut chars)))
            }
            c if c.is_whitespace() => {}
            _ => return Err(LexError::UnexpectedChar(c)),
        }
    }
    Ok(tokens)
}

fn read_number(first: char, chars: &mut Peekable<Chars>) -> i64 {
    let mut number = first.to_string();
    while let Some(&c) = chars.peek() {
        if !c.is_ascii_digit() {
            break;
        }
        number.push(c);
        chars.next();
    }
    number.parse().unwrap()
}

fn read_word(first: char, chars: &mut Peekable<Chars>) -> String {
    let mut string = first.to_string();
    while let Some(&c) = chars.peek() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_single_chars() {
        assert_eq!(
            lex("= + ;"),
            Ok(vec![Token::Equals, Token::Plus, Token::Semicolon])
        );
    }

    #[test]
    fn lexes_numbers() {
        assert_eq!(
            lex("42 + 7;"),
            Ok(vec![
                Token::Number(42),
                Token::Plus,
                Token::Number(7),
                Token::Semicolon
            ])
        );
    }

    #[test]
    fn lexes_identifiers() {
        assert_eq!(
            lex("x + abc;"),
            Ok(vec![
                Token::Ident("x".into()),
                Token::Plus,
                Token::Ident("abc".into()),
                Token::Semicolon
            ])
        );
    }

    #[test]
    fn lexes_let_statements() {
        assert_eq!(
            lex("let x = 42 + y;"),
            Ok(vec![
                Token::Let,
                Token::Ident("x".into()),
                Token::Equals,
                Token::Number(42),
                Token::Plus,
                Token::Ident("y".into()),
                Token::Semicolon
            ])
        );
    }

    #[test]
    fn lexes_identifiers_with_digits() {
        assert_eq!(
            lex("x1 + abc;"),
            Ok(vec![
                Token::Ident("x1".into()),
                Token::Plus,
                Token::Ident("abc".into()),
                Token::Semicolon
            ])
        );
    }

    #[test]
    fn lexes_identifiers_with_underscore() {
        assert_eq!(
            lex("my_var;"),
            Ok(vec![Token::Ident("my_var".into()), Token::Semicolon])
        );
    }

    #[test]
    fn lexes_operators_and_parens() {
        assert_eq!(
            lex("let z = (x - 1) * 2 / y;"),
            Ok(vec![
                Token::Let,
                Token::Ident("z".into()),
                Token::Equals,
                Token::LParen,
                Token::Ident("x".into()),
                Token::Minus,
                Token::Number(1),
                Token::RParen,
                Token::Star,
                Token::Number(2),
                Token::Slash,
                Token::Ident("y".into()),
                Token::Semicolon,
            ])
        );
    }
    #[test]
    fn rejects_unknown_char() {
        assert_eq!(lex("x @ y"), Err(LexError::UnexpectedChar('@')));
    }
}

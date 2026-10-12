use std::iter::Peekable;
use std::str::CharIndices;

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

#[cfg(test)]
mod tests {
    use super::Token::*;
    use super::*;

    #[test]
    fn lexes_single_chars() {
        assert_eq!(lex("= + ;"), Ok(vec![Equals, Plus, Semicolon]));
    }

    #[test]
    fn lexes_numbers() {
        assert_eq!(
            lex("42 + 7;"),
            Ok(vec![Number(42), Plus, Number(7), Semicolon])
        );
    }

    #[test]
    fn lexes_identifiers() {
        assert_eq!(
            lex("x + abc;"),
            Ok(vec![
                Ident("x".into()),
                Plus,
                Ident("abc".into()),
                Semicolon
            ])
        );
    }

    #[test]
    fn lexes_let_statements() {
        assert_eq!(
            lex("let x = 42 + y;"),
            Ok(vec![
                Let,
                Ident("x".into()),
                Equals,
                Number(42),
                Plus,
                Ident("y".into()),
                Semicolon
            ])
        );
    }

    #[test]
    fn lexes_identifiers_with_digits() {
        assert_eq!(
            lex("x1 + abc;"),
            Ok(vec![
                Ident("x1".into()),
                Plus,
                Ident("abc".into()),
                Semicolon
            ])
        );
    }

    #[test]
    fn lexes_identifiers_with_underscore() {
        assert_eq!(lex("my_var;"), Ok(vec![Ident("my_var".into()), Semicolon]));
    }

    #[test]
    fn lexes_operators_and_parens() {
        assert_eq!(
            lex("let z = (x - 1) * 2 / y;"),
            Ok(vec![
                Let,
                Ident("z".into()),
                Equals,
                LParen,
                Ident("x".into()),
                Minus,
                Number(1),
                RParen,
                Star,
                Number(2),
                Slash,
                Ident("y".into()),
                Semicolon,
            ])
        );
    }
    #[test]
    fn rejects_unknown_char() {
        assert_eq!(
            lex("x @ y"),
            Err(LexError::UnexpectedChar { ch: '@', pos: 2 })
        );
    }

    #[test]
    fn number_too_large() {
        assert_eq!(
            lex("x = 99999999999999999999;"),
            Err(LexError::NumberTooLarge { pos: 4 })
        );
    }

    #[test]
    fn empty_and_whitespace_only_input() {
        assert_eq!(lex(""), Ok(vec![]));
        assert_eq!(lex(" \t\r\n"), Ok(vec![]));
    }

    #[test]
    fn whitespace_between_tokens_is_optional() {
        assert_eq!(
            lex("let\tx=(1+y);\n"),
            Ok(vec![
                Let,
                Ident("x".into()),
                Equals,
                LParen,
                Number(1),
                Plus,
                Ident("y".into()),
                RParen,
                Semicolon,
            ])
        );
    }

    #[test]
    fn keyword_only_matches_a_whole_identifier() {
        assert_eq!(
            lex("letter let_ _let let1 Let"),
            Ok(vec![
                Ident("letter".into()),
                Ident("let_".into()),
                Ident("_let".into()),
                Ident("let1".into()),
                Ident("Let".into()),
            ])
        );
    }

    #[test]
    fn largest_i64_is_accepted() {
        assert_eq!(lex("9223372036854775807"), Ok(vec![Number(i64::MAX)]));
    }

    #[test]
    fn smallest_overflow_is_rejected() {
        assert_eq!(
            lex("9223372036854775808"),
            Err(LexError::NumberTooLarge { pos: 0 })
        );
    }

    #[test]
    fn umlaut_position_is_in_bytes() {
        // ö and ß are 2 bytes each in UTF-8, so '$' is at byte 10, not char 8
        assert_eq!(
            lex("größe = $;"),
            Err(LexError::UnexpectedChar { ch: '$', pos: 10 })
        );
    }
    #[test]
    fn equals_vs_eqeq() {
        assert_eq!(lex("x = 5"), Ok(vec![Ident("x".into()), Equals, Number(5)]));
        assert_eq!(lex("x == 5"), Ok(vec![Ident("x".into()), EqEq, Number(5)]));
    }

    #[test]
    fn comparison_operators() {
        assert_eq!(
            lex("a < b <= c > d >= e != f"),
            Ok(vec![
                Ident("a".into()),
                Less,
                Ident("b".into()),
                LessEq,
                Ident("c".into()),
                Greater,
                Ident("d".into()),
                GreaterEq,
                Ident("e".into()),
                NotEq,
                Ident("f".into()),
            ])
        );
    }

    #[test]
    fn lone_bang_is_an_error() {
        assert_eq!(
            lex("a ! b"),
            Err(LexError::UnexpectedChar { ch: '!', pos: 2 })
        );
    }
}

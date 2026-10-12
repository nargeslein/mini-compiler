use crate::token::Token::*;
use crate::token::{LexError, Token};
use crate::{ai_generated, handwritten};

type LexFn = fn(&str) -> Result<Vec<Token>, LexError>;

const LEXERS: [(&str, LexFn); 2] = [
    ("handwritten", handwritten::lex),
    ("ai_generated", ai_generated::lex),
];

/// Checks an input against both lexers.
fn check(input: &str, expected: Result<Vec<Token>, LexError>) {
    for (name, lex) in LEXERS {
        assert_eq!(lex(input), expected, "lexer: {name}, input: {input:?}");
    }
}

fn ident(name: &str) -> Token {
    Ident(name.to_string())
}

#[test]
fn let_statement() {
    check(
        "let x = 42 + y;",
        Ok(vec![
            Let,
            ident("x"),
            Equals,
            Number(42),
            Plus,
            ident("y"),
            Semicolon,
        ]),
    );
}

#[test]
fn equals_vs_eqeq() {
    check("x = 5", Ok(vec![ident("x"), Equals, Number(5)]));
    check("x == 5", Ok(vec![ident("x"), EqEq, Number(5)]));
}

#[test]
fn lone_bang_is_an_error() {
    check("a ! b", Err(LexError::UnexpectedChar { ch: '!', pos: 2 }));
}
#[test]
fn lexes_single_chars() {
    check("= + ;", Ok(vec![Equals, Plus, Semicolon]));
}

#[test]
fn lexes_numbers() {
    check("42 + 7;", Ok(vec![Number(42), Plus, Number(7), Semicolon]));
}

#[test]
fn lexes_identifiers() {
    check(
        "x + abc;",
        Ok(vec![ident("x"), Plus, ident("abc"), Semicolon]),
    );
}

#[test]
fn lexes_identifiers_with_digits() {
    check(
        "x1 + abc;",
        Ok(vec![ident("x1"), Plus, ident("abc"), Semicolon]),
    );
}

#[test]
fn lexes_identifiers_with_underscore() {
    check("my_var;", Ok(vec![ident("my_var"), Semicolon]));
}

#[test]
fn lexes_operators_and_parens() {
    check(
        "let z = (x - 1) * 2 / y;",
        Ok(vec![
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
        ]),
    );
}
#[test]
fn rejects_unknown_char() {
    check("x @ y", Err(LexError::UnexpectedChar { ch: '@', pos: 2 }));
}

#[test]
fn number_too_large() {
    check(
        "x = 99999999999999999999;",
        Err(LexError::NumberTooLarge { pos: 4 }),
    );
}

#[test]
fn empty_and_whitespace_only_input() {
    check("", Ok(vec![]));
    check(" \t\r\n", Ok(vec![]));
}

#[test]
fn whitespace_between_tokens_is_optional() {
    check(
        "let\tx=(1+y);\n",
        Ok(vec![
            Let,
            ident("x"),
            Equals,
            LParen,
            Number(1),
            Plus,
            ident("y"),
            RParen,
            Semicolon,
        ]),
    );
}

#[test]
fn keyword_only_matches_a_whole_identifier() {
    check(
        "letter let_ _let let1 Let",
        Ok(vec![
            ident("letter"),
            ident("let_"),
            ident("_let"),
            ident("let1"),
            ident("Let"),
        ]),
    );
}

#[test]
fn largest_i64_is_accepted() {
    check("9223372036854775807", Ok(vec![Number(i64::MAX)]));
}

#[test]
fn smallest_overflow_is_rejected() {
    check(
        "9223372036854775808",
        Err(LexError::NumberTooLarge { pos: 0 }),
    );
}

#[test]
fn umlaut_position_is_in_bytes() {
    // The lexers intentionally behave differently here:
    // - handwritten allows Unicode in identifiers. ö and ß are 2 bytes each
    //   in UTF-8, so '$' is at byte 10, not char 8.
    // - ai_generated only allows ASCII in identifiers and stops at the 'ö'
    //   at byte 2.
    let input = "größe = $;";
    for (name, lex) in LEXERS {
        let expected = match name {
            "handwritten" => Err(LexError::UnexpectedChar { ch: '$', pos: 10 }),
            "ai_generated" => Err(LexError::UnexpectedChar { ch: 'ö', pos: 2 }),
            _ => unreachable!("no expected result for lexer {name}"),
        };
        assert_eq!(lex(input), expected, "lexer: {name}, input: {input:?}");
    }
}

#[test]
fn comparison_operators() {
    check(
        "a < b <= c > d >= e != f",
        Ok(vec![
            ident("a"),
            Less,
            ident("b"),
            LessEq,
            ident("c"),
            Greater,
            ident("d"),
            GreaterEq,
            ident("e"),
            NotEq,
            ident("f"),
        ]),
    );
}

// --- Tests by Claude (moved over from the old ai_generated.rs test suite) ---

#[test]
fn claude_operators_are_matched_greedily() {
    // `===` is `==` followed by `=`, `<==` is `<=` followed by `=`
    check("===", Ok(vec![EqEq, Equals]));
    check("<==", Ok(vec![LessEq, Equals]));
}

#[test]
fn claude_space_splits_two_char_operators() {
    check("= =", Ok(vec![Equals, Equals]));
    check("< =", Ok(vec![Less, Equals]));
    check("> =", Ok(vec![Greater, Equals]));
}

#[test]
fn claude_comparison_operators_without_whitespace() {
    check(
        "a<b>=c!=d==e>f",
        Ok(vec![
            ident("a"),
            Less,
            ident("b"),
            GreaterEq,
            ident("c"),
            NotEq,
            ident("d"),
            EqEq,
            ident("e"),
            Greater,
            ident("f"),
        ]),
    );
}

#[test]
fn claude_bang_at_end_of_input_is_an_error() {
    check("x !", Err(LexError::UnexpectedChar { ch: '!', pos: 2 }));
}

#[test]
fn claude_bang_followed_by_space_and_equals_is_an_error() {
    check("a ! = b", Err(LexError::UnexpectedChar { ch: '!', pos: 2 }));
}

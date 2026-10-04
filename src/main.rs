use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum Token {
    Let,
    Ident(String),
    Number(i64),
    Plus,
    Equals,
    Semicolon,
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '=' => tokens.push(Token::Equals),
            '+' => tokens.push(Token::Plus),
            ';' => tokens.push(Token::Semicolon),
            '0'..='9' => tokens.push(Token::Number(read_number(c, &mut chars))),
            c if c.is_alphabetic() || c == '_' => {
                tokens.push(match_token(read_word(c, &mut chars)))
            }
            c if c.is_whitespace() => {}
            _ => {}
        }
    }
    tokens
}

fn main() {
    let tokens = lex("= + ;");
    println!("{:?}", tokens);
    let tokens2 = lex("42 + 7;");
    println!("{:?}", tokens2);
    let tokens3 = lex("x + abc;");
    println!("{:?}", tokens3);
    let tokens4 = lex("x1 + abc;");
    println!("{:?}", tokens4);
    let tokens5 = lex("my_var;");
    println!("{:?}", tokens5);
    let tokens6 = lex("let x = 42 + y;");
    println!("{:?}", tokens6);
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
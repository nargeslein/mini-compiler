# mini-compiler

A small compiler written in Rust, built step by step and in public.

Part one is the **lexer**: it turns source text like

```
let z = (x - 1) * 2 / y;
```

into a list of tokens:

```
[Let, Ident("z"), Equals, LParen, Ident("x"), Minus, Number(1), RParen,
 Star, Number(2), Slash, Ident("y"), Semicolon]
```

## Why this project

- **Turn compiler theory into working code.** Lexer, parser, evaluation, one stage at a time.
- **Compare hand-written and AI-assisted code** on the same problem, with the same tests.
- **Deepen my Rust skills by building**, not just reading.

## Two lexers, one problem

This repo contains two implementations of the same lexer. Both use the same token and error types and run against one shared test suite.

| | Hand-written | AI-assisted (Claude Code) |
| --- | --- | --- |
| Time to write | about 1 hour | about 6 minutes |
| Helper functions | `read_number`, `read_word` and `read_operator` | one generic `lexeme` helper taking a closure |
| Two-character operators | `read_operator` looks one character ahead with `next_if` and matches on `(char, bool)` | handled inside the main `match` |
| Error handling | returns `Result<Vec<Token>, LexError>` | returns `Result<Vec<Token>, LexError>` |
| How errors are returned | `map_err` and `?` | `return Err(...)` inside the `match` |
| Unknown characters | `Err(UnexpectedChar { ch, pos })` | `Err(UnexpectedChar { ch, pos })` |
| Number overflow | `Err(NumberTooLarge { pos })` | `Err(NumberTooLarge { pos })` |
| Identifiers | any Unicode letter (`größe` works) | ASCII only (`größe` fails at `ö`) |

Both pass the same shared tests. The AI version came first with many of the edge cases, for example that `===` is lexed as `==` followed by `=`. I ported those tests, and in turn the hand-written lexer was the first to report errors as values instead of panicking.

There is exactly one test where the two lexers intentionally disagree: identifiers with umlauts. That test lists the expected result for each lexer separately.

The point is not who is faster. To judge what the AI changed, which edge cases it added, why it uses lifetimes, or that it quietly restricted identifiers to ASCII, I first had to solve the problem myself. **Not speed, but judgment.**

What would you change in the hand-written version? Issues and comments are welcome.

## Token set

| Token | Example |
| --- | --- |
| `Let` | `let` |
| `Ident(String)` | `x`, `my_var`, `x1` |
| `Number(i64)` | `42` |
| `Plus`, `Minus`, `Star`, `Slash` | `+ - * /` |
| `Equals` | `=` |
| `EqEq`, `NotEq` | `== !=` |
| `Less`, `LessEq`, `Greater`, `GreaterEq` | `< <= > >=` |
| `Semicolon` | `;` |
| `LParen`, `RParen` | `( )` |

Identifiers start with a letter or underscore and may contain letters, digits and underscores. `let` is a keyword. Whitespace is ignored. Two-character operators are matched greedily: `===` is `==` followed by `=`. A lone `!` is an error.

## Errors

```rust
pub enum LexError {
    UnexpectedChar { ch: char, pos: usize },
    NumberTooLarge { pos: usize },
}
```

`pos` is the byte offset in the input where the problem starts:

```rust
lex("x = 4 $ 2;")                 // Err(UnexpectedChar { ch: '$', pos: 6 })
lex("x = 99999999999999999999;")  // Err(NumberTooLarge { pos: 4 })
lex("a ! b")                      // Err(UnexpectedChar { ch: '!', pos: 2 })
```

## Project structure

```
src/
  main.rs           entry point, runs both lexers on the same input
  token.rs          shared Token and LexError types
  handwritten.rs    my hand-written lexer
  ai_generated.rs   Claude Code's lexer
  shared_tests.rs   one test suite, every test runs against both lexers
```

## Getting started

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/nargeslein/mini-compiler.git
cd mini-compiler
cargo run     # prints the tokens from both lexers
cargo test    # runs the shared test suite against both lexers
```

## Roadmap

- [x] Lexer: numbers, identifiers, `let`, operators, parentheses
- [x] Unit tests
- [x] Hand-written lexer returns a `Result` with proper error types
- [x] AI-generated lexer returns the same `Result` instead of panicking
- [x] Comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`)
- [x] Shared `Token` and `LexError` types and one shared test suite for both lexers
- [ ] Parser: build a syntax tree from tokens
- [ ] Evaluation: run simple programs like `let x = 42 + 7; x * 2`

## About

Built by [Narges Hadji-Hosseini](https://www.linkedin.com/in/narges-hadji-hosseini), software engineer, as part of a learning-in-public series on Rust and AI-assisted coding, and as a small taste of what [TechGarden Frankfurt](https://www.techgarden-frankfurt.de/) is about: builders sitting side by side, arguing about edge cases and shipping.

**Build. Code. Ship.**
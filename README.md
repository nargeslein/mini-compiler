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

This repo contains two implementations of the same lexer with the same public interface:

| | Hand-written | AI-assisted (Claude Code) |
| --- | --- | --- |
| Time to write | about 1 hour | about 6 minutes |
| Reading words and numbers | builds a new `String` char by char | slices directly from the input using byte offsets and lifetimes |
| Helper functions | `read_number` and `read_word` | one generic `lexeme` helper taking a closure |
| Unknown characters | silently ignored | panic with position |
| Number overflow | `unwrap` panics | panic with a clear message |
| Identifiers | any Unicode letter (`größe` works) | ASCII only |
| Tests | 7 core cases | 7 core cases plus edge cases |

Both pass the same core tests. The AI version is more thorough and closer to production. The hand-written version is easier to read.

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
| `Semicolon` | `;` |
| `LParen`, `RParen` | `( )` |

Identifiers start with a letter or underscore and may contain letters, digits and underscores. `let` is a keyword. Whitespace is ignored.

## Project structure

```
src/
  main.rs           entry point, runs both lexers on the same input
  handwritten.rs    my hand-written lexer and tests
  ai_generated.rs   Claude Code's lexer and tests
```

## Getting started

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/nargeslein/mini-compiler.git
cd mini-compiler
cargo run     # prints the tokens from both lexers
cargo test    # runs the tests for both lexers
```

## Roadmap

- [x] Lexer: numbers, identifiers, `let`, operators, parentheses
- [x] Unit tests
- [ ] Return a `Result` with proper error types instead of panicking or ignoring input
- [ ] Shared `Token` type and a `Lexer` trait so both implementations share one test suite
- [ ] Parser: build a syntax tree from tokens
- [ ] Evaluation: run simple programs like `let x = 42 + 7; x * 2`

## About

Built by [Narges Hadji-Hosseini](https://www.linkedin.com/in/), software engineer, as part of a learning-in-public series on Rust and AI-assisted coding, and as a small taste of what [TechGarden Frankfurt](https://www.techgarden-frankfurt.de/) is about: builders sitting side by side, arguing about edge cases and shipping.

**Build. Code. Ship.**

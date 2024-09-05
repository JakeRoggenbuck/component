use crate::lexer::{Lex, Lexer, Token, TokenType};
use efcl::{bold, color, Color};
use parse::parse;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

pub mod lexer;
pub mod parse;

fn interactive() {
    let verbose = false;

    let mut local_memory = HashMap::new();

    local_memory.insert(
        "e".to_string(),
        Token {
            token_type: TokenType::NumericDecLiteral,
            value: std::f64::consts::E.to_string(),
        },
    );

    local_memory.insert(
        "pi".to_string(),
        Token {
            token_type: TokenType::NumericDecLiteral,
            value: std::f64::consts::PI.to_string(),
        },
    );

    local_memory.insert(
        "C".to_string(),
        Token {
            token_type: TokenType::NumericIntLiteral,
            value: "299792458".to_string(),
        },
    );

    loop {
        print!("{}", color!(Color::GREEN, bold!("\n> ").as_str()));
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        let _ = stdin().read_line(&mut input);

        let mut lex: Lexer = Lexer::new(vec![input]);

        let mut tokens = Vec::<Token>::new();

        loop {
            let a = lex.next();

            if a.token_type == TokenType::EndToken {
                break;
            }

            if verbose {
                println!(
                    "{}",
                    color!(Color::BLACK, format!("Stack before: {:?}", a).as_str())
                );
            }
            tokens.push(a);
        }

        let out = parse(tokens, &mut local_memory, verbose);
        if out.token_type != TokenType::NoType {
            println!(
                "{} {}",
                color!(Color::GREEN, bold!("->").as_str()),
                color!(Color::BLUE, bold!(&out.value).as_str())
            );
        }
    }
}

fn main() {
    interactive();
}

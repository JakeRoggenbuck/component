use crate::lexer::{Lex, Lexer, Token, TokenType};
use efcl::{bold, color, Color};
use parse::{create_parser, Parser};
use std::io::{stdin, stdout, Write};

pub mod lexer;
pub mod parse;

fn interactive() {
    let verbose = false;
    let mut p = create_parser(verbose);

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
                    color!(Color::BLACK, format!("Lexing Token: {:?}", a).as_str())
                );
            }
            tokens.push(a);
        }

        if verbose {
            println!("{}", color!(Color::BLACK, "End of lexing"));
        }

        let out = p.parse(tokens);

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

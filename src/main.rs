use crate::lexer::{Lex, Lexer, Token, TokenType};
use efcl::{bold, color, Color};
use parse::parse;
use std::io::{stdin, stdout, Write};

pub mod lexer;
pub mod parse;

fn interactive() {
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

            println!("{:?}", a);
            tokens.push(a);
        }

        let out = parse(tokens);
        println!("{}", out.value);
    }
}

fn main() {
    interactive();
}

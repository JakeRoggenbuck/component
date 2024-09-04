use crate::lexer::{Lex, Lexer, Token, TokenType};
use efcl::{bold, color, Color};
use std::io::{stdin, stdout, Write};

pub mod lexer;

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
    }
}

fn main() {
    interactive();
}

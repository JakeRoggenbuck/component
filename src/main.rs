use crate::lexer::{Lex, Lexer, Token, TokenType};
use efcl::{bold, color, Color};
use parse::{create_parser, Parser};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::{stdin, stdout, Write};
use structopt::StructOpt;

pub mod display;
pub mod lexer;
pub mod parse;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,

    #[structopt(short, long)]
    filename: Option<String>,
}

fn run_file(filename: String, verbose: bool) {
    let mut p = create_parser(verbose);

    let file_res = File::open(filename);
    match file_res {
        Ok(file) => {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        println!("{}{}", color!(Color::GREEN, bold!("\n> ").as_str()), l);

                        let mut lex = Lexer::new(vec![l]);

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
                    Err(_) => {}
                }
            }
        }
        Err(_) => {}
    }
}

fn interactive(verbose: bool) {
    let mut p = create_parser(verbose);

    loop {
        print!("{}", color!(Color::GREEN, bold!("\n> ").as_str()));
        stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        let _ = stdin().read_line(&mut input);

        let mut lex = Lexer::new(vec![input]);

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
    let opt = Opt::from_args();

    if let Some(filename) = opt.filename {
        run_file(filename, opt.verbose);
    } else {
        interactive(opt.verbose);
    }
}

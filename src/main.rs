use crate::lexer::{Lex, Lexer, Token, TokenType};
use efcl::{bold, color, Color};
use parse::{create_parser, AssemblyArchitecture, Parser};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::io::{stdin, stdout, Write};
use structopt::StructOpt;

pub mod display;
pub mod lexer;
pub mod parse;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Component",
    about = "A programming language for math using postfix notation"
)]
struct Opt {
    /// Print the stack and lexer information on each command
    #[structopt(short, long)]
    verbose: bool,

    /// Use `x86` or `x86-64` to get x86-64 assembly and use `RISCV` or `RISC-V` for RISC-V assembly
    #[structopt(short, long)]
    asm: Option<String>,

    /// Specify an input Component file to be run
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

fn interactive(verbose: bool, asm: Option<AssemblyArchitecture>) {
    let mut p = create_parser(verbose);

    if let Some(a) = asm {
        p.set_asm_arch(a);
    }

    println!(
        "\n{} {} -- {}",
        color!(Color::YELLOW, bold!(">").as_str()),
        bold!("Component").as_str(),
        bold!("v0.1.0").as_str()
    );
    println!("------------------------------------------------------");
    println!("A programming language for math using postfix notation");
    println!("-- Source: https://github.com/JakeRoggenbuck/component");
    println!(
        "-- Use: {} for the Component guide and {} to exit.",
        color!(Color::GREEN, bold!("??").as_str()),
        color!(Color::GREEN, bold!("Ctrl+c").as_str()),
    );

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

        if asm.is_some() {
            for x in p.output_asm() {
                println!("{}", color!(Color::BLACK, x.as_str()));
            }
        }

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
        let mut asm = None;
        if opt.asm == Some("x86".to_string()) || opt.asm == Some("x86-64".to_string()) {
            asm = Some(AssemblyArchitecture::X86_64);
        } else if opt.asm == Some("RISCV".to_string()) || opt.asm == Some("RISC-V".to_string()) {
            asm = Some(AssemblyArchitecture::RISCV);
        }

        interactive(opt.verbose, asm);
    }
}

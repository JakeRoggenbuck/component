use super::lexer::{Lex, Lexer, Token, TokenType};

pub fn parse(tokens: Vec<Token>) {
    let mut stack: Vec<Token> = vec![];

    println!("Stack before: {:?}", stack);

    // Parse postfix notation
    for token in tokens {
        // Match the type of token
        // if it's a literal, add it to the stack
        // if it's an operation, pop values from the stack and apply the operation
        match token.token_type {
            TokenType::NumericIntLiteral => {
                stack.push(token);
            }

            // Simple two argument operations/functions
            TokenType::Addition
            | TokenType::Multiplication
            | TokenType::Subtraction
            | TokenType::Division => {
                let first = stack.pop();
                let second = stack.pop();

                // Check that both items poped from the stack actually exist
                // i.e. there are enough items on the stack
                match (first, second) {
                    // Both items exist on the stack
                    (Some(a), Some(b)) => {
                        // Check the type of the two items poped from the stack
                        match (a.token_type, b.token_type) {
                            // Both token are NumericIntLiteral
                            (TokenType::NumericIntLiteral, TokenType::NumericIntLiteral) => {
                                let a_int_res = a.value.parse::<i32>();
                                let b_int_res = b.value.parse::<i32>();

                                // Check that both numbers parsed correctly
                                match (a_int_res, b_int_res) {
                                    // Both values parsed correctly
                                    (Ok(a_int), Ok(b_int)) => match token.token_type {
                                        TokenType::Addition => stack.push(Token {
                                            token_type: TokenType::NumericIntLiteral,
                                            value: (a_int + b_int).to_string(),
                                        }),
                                        TokenType::Multiplication => stack.push(Token {
                                            token_type: TokenType::NumericIntLiteral,
                                            value: (a_int * b_int).to_string(),
                                        }),
                                        TokenType::Subtraction => stack.push(Token {
                                            token_type: TokenType::NumericIntLiteral,
                                            value: (a_int - b_int).to_string(),
                                        }),
                                        TokenType::Division => stack.push(Token {
                                            token_type: TokenType::NumericIntLiteral,
                                            value: (a_int / b_int).to_string(),
                                        }),
                                        _ => {
                                            unreachable!();
                                        }
                                    },

                                    // Give errors if values did not parse correctly
                                    (Err(_), Ok(_)) => {
                                        println!("{} {} {}", a.value, b.value, token.value);
                                        println!("^ value is not a NumericIntLiteral because it did not parse correcly");
                                    }
                                    (Ok(_), Err(_)) => {
                                        println!("{} {} {}", a.value, b.value, token.value);
                                        println!(
                                            "{}^value is not a NumericIntLiteral because it did not parse correcly",
                                            (0..a.value.len()).map(|_| " ").collect::<String>()
                                        );
                                    }
                                    (Err(_), Err(_)) => {
                                        println!("{} {} {}", a.value, b.value, token.value);
                                        println!(
                                            "^{}^ values are not a NumericIntLiteral because they did not parse correcly",
                                            (0..a.value.len()).map(|_| " ").collect::<String>(),
                                        );
                                    }
                                }
                            }

                            // Give errors if values are not NumericIntLiteral
                            (_, TokenType::NumericIntLiteral) => {
                                println!("{} {} {}", a.value, b.value, token.value);
                                println!("^ value is not a NumericIntLiteral");
                            }
                            (TokenType::NumericIntLiteral, _) => {
                                println!("{} {} {}", a.value, b.value, token.value);
                                println!(
                                    "{}^value is not a NumericIntLiteral",
                                    (0..a.value.len()).map(|_| " ").collect::<String>()
                                );
                            }
                            (_, _) => {
                                println!("{} {} {}", a.value, b.value, token.value);
                                println!(
                                    "^{}^ values are not a NumericIntLiteral",
                                    (0..a.value.len()).map(|_| " ").collect::<String>(),
                                );
                            }
                        }
                    }
                    _ => {
                        println!("Not enough values on the stack.");
                    }
                }
            }

            _ => {
                println!("Operation not implemented.");
            }
        }
    }

    println!("Stack after: {:?}", stack);
}

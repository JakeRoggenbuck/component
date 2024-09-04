use super::lexer::{Token, TokenType};
use efcl::{bold, color, Color};

pub fn parse(tokens: Vec<Token>) -> Token {
    let mut stack: Vec<Token> = vec![];

    println!(
        "{}",
        color!(Color::BLACK, format!("Stack before: {:?}", stack).as_str())
    );

    // Parse postfix notation
    for token in tokens {
        // Match the type of token
        // if it's a literal, add it to the stack
        // if it's an operation, pop values from the stack and apply the operation
        match token.token_type {
            TokenType::NumericIntLiteral | TokenType::NumericDecLiteral | TokenType::Identifier => {
                stack.push(token);
            }

            // Simple two argument operations/functions
            TokenType::Addition
            | TokenType::Multiplication
            | TokenType::Subtraction
            | TokenType::Division => {
                let second = stack.pop();
                let first = stack.pop();

                // Check that both items poped from the stack actually exist
                // i.e. there are enough items on the stack
                match (first, second) {
                    // Both items exist on the stack
                    (Some(a), Some(b)) => {
                        // Check the type of the two items poped from the stack
                        match (a.token_type, b.token_type) {
                            // Both token are NumericIntLiteral or NumericDecLiteral
                            (m, n)
                                if (m == TokenType::NumericDecLiteral
                                    || m == TokenType::NumericIntLiteral)
                                    && (n == TokenType::NumericDecLiteral
                                        || n == TokenType::NumericIntLiteral) =>
                            {
                                let a_int_res = a.value.parse::<f64>();
                                let b_int_res = b.value.parse::<f64>();

                                // Check that both numbers parsed correctly
                                match (a_int_res, b_int_res) {
                                    // Both values parsed correctly
                                    (Ok(a_int), Ok(b_int)) => match token.token_type {
                                        TokenType::Addition => {
                                            let v = a_int + b_int;
                                            let t_type = if v.fract() == 0.0 {
                                                TokenType::NumericIntLiteral
                                            } else {
                                                TokenType::NumericDecLiteral
                                            };
                                            stack.push(Token {
                                                token_type: t_type,
                                                value: v.to_string(),
                                            })
                                        }
                                        TokenType::Multiplication => {
                                            let v = a_int * b_int;
                                            let t_type = if v.fract() == 0.0 {
                                                TokenType::NumericIntLiteral
                                            } else {
                                                TokenType::NumericDecLiteral
                                            };
                                            stack.push(Token {
                                                token_type: t_type,
                                                value: v.to_string(),
                                            });
                                        }
                                        TokenType::Subtraction => {
                                            let v = a_int - b_int;
                                            let t_type = if v.fract() == 0.0 {
                                                TokenType::NumericIntLiteral
                                            } else {
                                                TokenType::NumericDecLiteral
                                            };
                                            stack.push(Token {
                                                token_type: t_type,
                                                value: v.to_string(),
                                            });
                                        }
                                        TokenType::Division => {
                                            let v = a_int / b_int;
                                            let t_type = if v.fract() == 0.0 {
                                                TokenType::NumericIntLiteral
                                            } else {
                                                TokenType::NumericDecLiteral
                                            };
                                            stack.push(Token {
                                                token_type: t_type,
                                                value: v.to_string(),
                                            });
                                        }
                                        _ => {
                                            unreachable!();
                                        }
                                    },

                                    // Give errors if values did not parse correctly
                                    (Err(_), Ok(_)) => {
                                        println!(
                                            "{} Wrong type",
                                            color!(Color::RED, bold!("Error:").as_str()).as_str()
                                        );
                                        println!(
                                            "{} {} {}",
                                            color!(Color::RED, bold!(a.value.as_str()).as_str()),
                                            b.value,
                                            token.value
                                        );
                                        println!(
                                            "{} value is not a Numeric because it did not parse correcly", 
                                            color!(Color::RED, bold!("^").as_str())
                                        );
                                    }
                                    (Ok(_), Err(_)) => {
                                        println!(
                                            "{} Wrong type",
                                            color!(Color::RED, bold!("Error:").as_str()).as_str()
                                        );
                                        println!(
                                            "{} {} {}",
                                            a.value,
                                            color!(Color::RED, bold!(b.value.as_str()).as_str()),
                                            token.value
                                        );
                                        println!(
                                            "{}{}value is not a Numeric because it did not parse correcly",
                                            (0..a.value.len() + 1).map(|_| " ").collect::<String>(),
                                            color!(Color::RED, bold!("^").as_str())
                                        );
                                    }
                                    (Err(_), Err(_)) => {
                                        println!(
                                            "{} Wrong type",
                                            color!(Color::RED, bold!("Error:").as_str()).as_str()
                                        );
                                        println!(
                                            "{} {} {}",
                                            color!(Color::RED, bold!(a.value.as_str()).as_str()),
                                            color!(Color::RED, bold!(b.value.as_str()).as_str()),
                                            token.value
                                        );
                                        println!(
                                            "{}{}{} values are not a Numeric because they did not parse correcly",
                                            color!(Color::RED, bold!("^").as_str()),
                                            (0..a.value.len()).map(|_| " ").collect::<String>(),
                                            color!(Color::RED, bold!("^").as_str())
                                        );
                                    }
                                }
                            }

                            // Give errors if values are not NumericIntLiteral or NumericDecLiteral
                            (_, TokenType::NumericIntLiteral) => {
                                println!(
                                    "{} Wrong type",
                                    color!(Color::RED, bold!("Error:").as_str()).as_str()
                                );
                                println!(
                                    "{} {} {}",
                                    color!(Color::RED, bold!(a.value.as_str()).as_str()),
                                    b.value,
                                    token.value
                                );
                                println!(
                                    "{} value is not a Numeric",
                                    color!(Color::RED, bold!("^").as_str())
                                );
                            }
                            (TokenType::NumericIntLiteral, _) => {
                                println!(
                                    "{} Wrong type",
                                    color!(Color::RED, bold!("Error:").as_str()).as_str()
                                );
                                println!(
                                    "{} {} {}",
                                    a.value,
                                    color!(Color::RED, bold!(b.value.as_str()).as_str()),
                                    token.value
                                );
                                println!(
                                    "{}{} value is not a Numeric",
                                    (0..a.value.len() + 1).map(|_| " ").collect::<String>(),
                                    color!(Color::RED, bold!("^").as_str())
                                );
                            }
                            (_, _) => {
                                println!(
                                    "{} Wrong type",
                                    color!(Color::RED, bold!("Error:").as_str()).as_str()
                                );
                                println!(
                                    "{} {} {}",
                                    color!(Color::RED, bold!(a.value.as_str()).as_str()),
                                    color!(Color::RED, bold!(b.value.as_str()).as_str()),
                                    token.value
                                );
                                println!(
                                    "{}{}{} values are not a Numeric",
                                    color!(Color::RED, bold!("^").as_str()),
                                    (0..a.value.len()).map(|_| " ").collect::<String>(),
                                    color!(Color::RED, bold!("^").as_str())
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

    println!(
        "{}",
        color!(Color::BLACK, format!("Stack after: {:?}", stack).as_str())
    );

    match stack.pop() {
        Some(a) => {
            return a;
        }
        None => {
            return Token {
                token_type: TokenType::NoType,
                value: "".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let input1 = vec![
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "1".to_string(),
            },
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "1".to_string(),
            },
            Token {
                token_type: TokenType::Addition,
                value: "+".to_string(),
            },
        ];

        let out1 = parse(input1);

        assert_eq!(
            out1,
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "2".to_string(),
            },
        );

        let input2 = vec![
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "3".to_string(),
            },
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "2".to_string(),
            },
            Token {
                token_type: TokenType::Multiplication,
                value: "*".to_string(),
            },
        ];

        let out2 = parse(input2);

        assert_eq!(
            out2,
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "6".to_string(),
            },
        );

        let input3 = vec![
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "3".to_string(),
            },
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "2".to_string(),
            },
            Token {
                token_type: TokenType::Addition,
                value: "+".to_string(),
            },
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "10".to_string(),
            },
            Token {
                token_type: TokenType::Multiplication,
                value: "*".to_string(),
            },
        ];

        let out3 = parse(input3);

        assert_eq!(
            out3,
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "50".to_string(),
            },
        );
    }
}

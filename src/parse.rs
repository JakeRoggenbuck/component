use super::lexer::{Token, TokenType};
use efcl::{bold, color, Color};
use std::collections::HashMap;

fn variable_check_pop(
    stack: &mut Vec<Token>,
    local_memory: &mut HashMap<String, Token>,
) -> Option<Token> {
    let first = stack.pop();

    if let Some(ref a) = first {
        match local_memory.get(&a.value) {
            Some(tok) => {
                return Some(tok.clone());
            }
            None => {
                return first;
            }
        }
    }

    return first;
}

pub fn parse(tokens: Vec<Token>, local_memory: &mut HashMap<String, Token>) -> Token {
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
            TokenType::NumericIntLiteral | TokenType::NumericDecLiteral => {
                stack.push(token);
            }

            TokenType::Identifier => match local_memory.get(&token.value) {
                // Push the value the variable is associated with
                Some(tok) => {
                    if stack.len() == 0 {
                        stack.push(tok.clone())
                    } else {
                        stack.push(token);
                    }
                }
                // Push the token of type Identifier as an Identifier to the stack
                None => stack.push(token),
            },

            // Create variables
            TokenType::Assignment => {
                // Use raw stack.pop here
                let second = stack.pop();
                let first = stack.pop();

                match (first, second) {
                    (Some(a), Some(b)) => {
                        // Does the variable already exist?

                        match local_memory.get(&b.value) {
                            Some(tok) => {
                                // Assigning to the same type as the existing variable
                                if tok.token_type == b.token_type {
                                    // Write variable to memory
                                    local_memory.insert(
                                        b.value,
                                        Token {
                                            token_type: a.token_type,
                                            value: a.value,
                                        },
                                    );
                                } else {
                                    println!(
                                        "{} Assignment Type Mismatch",
                                        color!(Color::RED, bold!("Error:").as_str()).as_str()
                                    );
                                    println!(
                                        "{} {} {}",
                                        a.value,
                                        color!(Color::RED, bold!(b.value.as_str()).as_str()),
                                        token.value
                                    );
                                    println!(
                                        "{}{} cannot assign value {} of type <{:?}> to a variable of type <{:?}>",
                                        (0..a.value.len() + 1).map(|_| " ").collect::<String>(),
                                        color!(
                                            Color::RED,
                                            bold!(&(0..a.value.len())
                                                .map(|_| "^")
                                                .collect::<String>())
                                            .as_str()
                                        ),
                                        a.value,
                                        a.token_type,
                                        tok.token_type,
                                    );
                                }
                            }
                            None => {
                                // Write variable to memory
                                local_memory.insert(
                                    b.value,
                                    Token {
                                        token_type: a.token_type,
                                        value: a.value,
                                    },
                                );
                            }
                        }
                    }
                    _ => {}
                }
            }

            TokenType::TypeIntKeyword => {
                let first = variable_check_pop(&mut stack, local_memory);

                match first {
                    Some(a) => match a.token_type {
                        TokenType::NumericIntLiteral | TokenType::NumericDecLiteral => {
                            let a_float_res = a.value.parse::<f64>();
                            match a_float_res {
                                Ok(a_val) => stack.push(Token {
                                    token_type: TokenType::NumericIntLiteral,
                                    value: (a_val as i64).to_string(),
                                }),
                                Err(_) => {
                                    println!(
                                        "{} Wrong type",
                                        color!(Color::RED, bold!("Error:").as_str()).as_str()
                                    );
                                    println!(
                                        "{} {}",
                                        color!(Color::RED, bold!(a.value.as_str()).as_str()),
                                        token.value
                                    );
                                    println!(
                                        "{} value is not a <NumericIntLiteral> or <NumericDecLiteral> because it did not parse correctly",
                                        color!(Color::RED, bold!("^").as_str())
                                    );
                                }
                            }
                        }

                        _ => {
                            println!(
                                "{} Invalid type cast",
                                color!(Color::RED, bold!("Error:").as_str()).as_str()
                            );
                            println!("{} {}", a.value, token.value);

                            println!(
                                "{}{} Cannot convert <{}> to <NumericIntLiteral>",
                                (0..a.value.len() + 1).map(|_| " ").collect::<String>(),
                                color!(Color::RED, bold!("^^^").as_str()),
                                format!("{:?}", a.token_type)
                            )
                        }
                    },
                    None => {
                        println!(
                            "{} Not enough values on the stack",
                            color!(Color::RED, bold!("Error:").as_str()).as_str()
                        );
                    }
                }
            }

            TokenType::TypeDecKeyword => {
                let first = variable_check_pop(&mut stack, local_memory);

                match first {
                    Some(a) => match a.token_type {
                        TokenType::NumericIntLiteral | TokenType::NumericDecLiteral => {
                            let a_float_res = a.value.parse::<f64>();
                            match a_float_res {
                                Ok(a_val) => stack.push(Token {
                                    token_type: TokenType::NumericDecLiteral,
                                    value: (a_val as f64).to_string(),
                                }),
                                Err(_) => {
                                    println!(
                                        "{} Wrong type",
                                        color!(Color::RED, bold!("Error:").as_str()).as_str()
                                    );
                                    println!(
                                        "{} {}",
                                        color!(Color::RED, bold!(a.value.as_str()).as_str()),
                                        token.value
                                    );
                                    println!(
                                        "{} value is not a <NumericIntLiteral> or <NumericDecLiteral> because it did not parse correctly",
                                        color!(Color::RED, bold!("^").as_str())
                                    );
                                }
                            }
                        }

                        _ => {
                            println!(
                                "{} Invalid type cast",
                                color!(Color::RED, bold!("Error:").as_str()).as_str()
                            );
                            println!("{} {}", a.value, token.value);

                            println!(
                                "{}{} Cannot convert <{}> to <NumericDecLiteral>",
                                (0..a.value.len() + 1).map(|_| " ").collect::<String>(),
                                color!(Color::RED, bold!("^^^").as_str()),
                                format!("{:?}", a.token_type)
                            )
                        }
                    },
                    None => {
                        println!(
                            "{} Not enough values on the stack",
                            color!(Color::RED, bold!("Error:").as_str()).as_str()
                        );
                    }
                }
            }

            // Simple two argument operations/functions
            TokenType::Addition
            | TokenType::Multiplication
            | TokenType::Subtraction
            | TokenType::Division => {
                let second = variable_check_pop(&mut stack, local_memory);
                let first = variable_check_pop(&mut stack, local_memory);

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
                                let a_float_res = a.value.parse::<f64>();
                                let b_float_res = b.value.parse::<f64>();

                                // Parse should keep (NumericIntLiteral NumericIntLiteral Operation) as an NumericIntLiteral
                                // if it's still a whole number and both inputs are ints
                                let keep_int = m == TokenType::NumericIntLiteral
                                    && n == TokenType::NumericIntLiteral;

                                // Check that both numbers parsed correctly
                                match (a_float_res, b_float_res) {
                                    // Both values parsed correctly
                                    (Ok(a_float), Ok(b_float)) => match token.token_type {
                                        TokenType::Addition => {
                                            let v = a_float + b_float;
                                            let t_type = if v.fract() == 0.0 && keep_int {
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
                                            let v = a_float * b_float;
                                            let t_type = if v.fract() == 0.0 && keep_int {
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
                                            let v = a_float - b_float;
                                            let t_type = if v.fract() == 0.0 && keep_int {
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
                                            let v = a_float / b_float;
                                            let t_type = if v.fract() == 0.0 && keep_int {
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
                                            "{} value is not a <NumericIntLiteral> or <NumericDecLiteral> because it did not parse correctly",
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
                                            "{}{}value is not a <NumericIntLiteral> or <NumericDecLiteral> because it did not parse correctly",
                                            (0..a.value.len() + 1).map(|_| " ").collect::<String>(),
                                            color!(Color::RED, bold!("^").as_str())
                                        )
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
                                            "{}{}{} values are not a <NumericIntLiteral> or <NumericDecLiteral> because they did not parse correctly",
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
                                    "{} value is not a <NumericIntLiteral> or <NumericDecLiteral>",
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
                                    "{}{} value is not a <NumericIntLiteral> or <NumericDecLiteral>",
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
                                    "{}{}{} values are not a <NumericIntLiteral> or <NumericDecLiteral>",
                                    color!(Color::RED, bold!("^").as_str()),
                                    (0..a.value.len()).map(|_| " ").collect::<String>(),
                                    color!(Color::RED, bold!("^").as_str())
                                );
                            }
                        }
                    }
                    _ => {
                        println!(
                            "{} Not enough values on the stack",
                            color!(Color::RED, bold!("Error:").as_str()).as_str()
                        );
                    }
                }
            }

            _ => {
                println!(
                    "{} Operation not implemented",
                    color!(Color::RED, bold!("Error:").as_str()).as_str()
                );
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

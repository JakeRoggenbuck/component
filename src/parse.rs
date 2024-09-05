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

pub fn parse(
    tokens: Vec<Token>,
    local_memory: &mut HashMap<String, Token>,
    function_memory: &mut HashMap<String, Vec<Token>>,
    verbose: bool,
) -> Token {
    let features_function = false;

    let mut stack: Vec<Token> = vec![];

    if verbose {
        println!(
            "{}",
            color!(Color::BLACK, format!("Stack before: {:?}", stack).as_str())
        );
    }

    // Parse postfix notation
    for token in tokens {
        if verbose {
            println!(
                "{}",
                color!(
                    Color::BLACK,
                    format!("Stack right now: {:?}", stack).as_str()
                )
            );
        }

        // Match the type of token
        // if it's a literal, add it to the stack
        // if it's an operation, pop values from the stack and apply the operation
        match token.token_type {
            TokenType::NumericIntLiteral | TokenType::NumericDecLiteral => {
                stack.push(token);
            }

            TokenType::Function => {
                // Experimental feature
                //
                // Ideas: Have a bool called function_stack and when you give a `>` character, the
                // parser goes into function stack mode where every symbol is put onto the function
                // stack and not evaluated. When the parser gets the keyword `func` it goes back to
                // regular mode. It then collects everything in the function stack and saves it in
                // the function_memory to be used later. It then clears the function memory and
                // continues to read commands.
                if features_function {
                    let func_name = variable_check_pop(&mut stack, local_memory);

                    if let Some(name) = func_name {
                        match function_memory.get(&name.value) {
                            Some(_) => {
                                println!(
                                    "{} Function Already Exists [E6]",
                                    color!(Color::RED, bold!("Error:").as_str()).as_str()
                                );
                                println!(
                                    "{}",
                                    color!(Color::RED, bold!(name.value.as_str()).as_str()),
                                );
                                println!(
                                    "{} function with the same name already exists",
                                    color!(
                                        Color::RED,
                                        bold!(&(0..name.value.len())
                                            .map(|_| "^")
                                            .collect::<String>())
                                        .as_str()
                                    ),
                                );
                            }
                            None => {
                                function_memory.insert(name.value, stack.clone());
                                println!("{:?}", stack.clone());
                                // Clear the function off the stack
                                stack = Vec::<Token>::new();
                            }
                        }
                    }
                } else {
                    println!(
                        "{} Operation Not Implemented [E5]",
                        color!(Color::RED, bold!("Error:").as_str()).as_str()
                    );
                }
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

            TokenType::TypeSqrtKeyword => {
                let first = variable_check_pop(&mut stack, local_memory);

                if let Some(a) = first {
                    let a_float_res = a.value.parse::<f64>();
                    match a_float_res {
                        Ok(a_val) => stack.push(Token {
                            token_type: TokenType::NumericIntLiteral,
                            value: ((a_val as f64).sqrt()).to_string(),
                        }),
                        Err(_) => {
                            println!(
                                "{} Wrong Type [E2]",
                                color!(Color::RED, bold!("Error:").as_str()).as_str()
                            );
                            println!(
                                "{} {}",
                                color!(Color::RED, bold!(a.value.as_str()).as_str()),
                                token.value
                            );
                            println!(
                                "{} value is not a <NumericIntLiteral> or <NumericDecLiteral>",
                                color!(Color::RED, bold!("^").as_str())
                            );
                        }
                    }
                } else {
                    println!(
                        "{} Stack Empty [E4]",
                        color!(Color::RED, bold!("Error:").as_str()).as_str()
                    );
                }
            }

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
                                if tok.token_type == a.token_type {
                                    // Write variable to memory
                                    let out = Token {
                                        token_type: a.token_type,
                                        value: a.value,
                                    };
                                    local_memory.insert(b.value, out.clone());
                                    stack.push(out);
                                } else {
                                    println!(
                                        "{} Assignment Type Mismatch [E1]",
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
                                let out = Token {
                                    token_type: a.token_type,
                                    value: a.value,
                                };
                                local_memory.insert(b.value, out.clone());
                                stack.push(out);
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
                                        "{} Wrong Type [E2]",
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
                                "{} Invalid Type Cast [E3]",
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
                            "{} Stack Empty [E4]",
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
                                        "{} Wrong Type [E2]",
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
                                "{} Invalid Type Cast [E3]",
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
                            "{} Stack Empty [E4]",
                            color!(Color::RED, bold!("Error:").as_str()).as_str()
                        );
                    }
                }
            }

            // Simple two argument operations/functions
            TokenType::Addition
            | TokenType::Multiplication
            | TokenType::Subtraction
            | TokenType::Division
            | TokenType::Carrot => {
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
                                        TokenType::Carrot => {
                                            let v = a_float.powf(b_float);
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
                                            "{} Wrong Type [E2]",
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
                                            "{} Wrong Type [E2]",
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
                                            "{} Wrong Type [E2]",
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
                                    "{} Wrong Type [E2]",
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
                                    "{} Wrong Type [E2]",
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
                                    "{} Wrong Type [E2]",
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
                            "{} Stack Empty [E4]",
                            color!(Color::RED, bold!("Error:").as_str()).as_str()
                        );
                    }
                }
            }

            _ => {
                println!(
                    "{} Operation Not Implemented [E5]",
                    color!(Color::RED, bold!("Error:").as_str()).as_str()
                );
            }
        }
    }

    if verbose {
        println!(
            "{}",
            color!(Color::BLACK, format!("Stack after: {:?}", stack).as_str())
        );
    }

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
        let mut local_memory = HashMap::new();
        let mut function_memory = HashMap::new();

        local_memory.insert(
            "e".to_string(),
            Token {
                token_type: TokenType::NumericDecLiteral,
                value: std::f64::consts::E.to_string(),
            },
        );

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

        let out1 = parse(input1, &mut local_memory, &mut function_memory, true);

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

        let out2 = parse(input2, &mut local_memory, &mut function_memory, true);

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

        let out3 = parse(input3, &mut local_memory, &mut function_memory, true);

        assert_eq!(
            out3,
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "50".to_string(),
            },
        );
    }
}

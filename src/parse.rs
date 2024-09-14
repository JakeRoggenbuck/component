use super::display::{
    invalid_type_cast_error, stack_empty_error, wrong_type_error_both, wrong_type_error_first,
    wrong_type_error_second,
};
use super::lexer::{Token, TokenType};
use efcl::{bold, color, Color};
use std::collections::HashMap;

pub trait Parser {
    fn variable_check_pop(&mut self) -> Option<Token>;
    fn match_token_type(&mut self, token: Token);
    fn parse(&mut self, tokens: Vec<Token>) -> Token;
    fn assign_value(&mut self, first: Option<Token>, second: Option<Token>, token: Token);
    fn convert_to_bool(&mut self, first: Option<Token>, token: Token);
    fn asm_li(&mut self, token: Token);
    fn asm_add(&mut self);
    fn asm_sub(&mut self);
    fn output_asm(&mut self) -> Vec<String>;
    fn reset_asm(&mut self);
}

#[derive(Debug)]
pub struct ParserState {
    function_mode: bool,
    verbose: bool,
    stack: Vec<Token>,
    token_stack: Vec<Token>,
    local_memory: HashMap<String, Token>,
    function_stack: Vec<Token>,
    function_memory: HashMap<String, Vec<Token>>,
    token_index: usize,
    assembly: Vec<String>,
    temp_reg_index: i8,
}

impl Parser for ParserState {
    fn output_asm(&mut self) -> Vec<String> {
        return self.assembly.clone();
    }

    fn reset_asm(&mut self) {
        self.assembly = vec![];
        self.temp_reg_index = 0;
    }

    fn convert_to_bool(&mut self, first: Option<Token>, token: Token) {
        match first {
            Some(a) => match a.token_type {
                TokenType::NumericIntLiteral
                | TokenType::NumericDecLiteral
                | TokenType::BoolLiteral => {
                    let a_float_res = a.value.parse::<f64>();
                    match a_float_res {
                        Ok(a_val) => self.stack.push(Token {
                            token_type: TokenType::BoolLiteral,
                            value: ((a_val != 0.0) as i32).to_string(),
                        }),
                        Err(_) => wrong_type_error_first(a.value, token.value),
                    }
                }
                _ => invalid_type_cast_error(String::from("BoolLiteral"), a, token),
            },
            None => stack_empty_error(),
        }
    }

    fn assign_value(&mut self, first: Option<Token>, second: Option<Token>, token: Token) {
        match (first, second) {
            (Some(a), Some(b)) => {
                // Does the variable already exist?
                match self.local_memory.get(&b.value) {
                    Some(tok) => {
                        // Assigning to the same type as the existing variable
                        if tok.token_type == a.token_type {
                            // Write variable to memory
                            let out = Token {
                                token_type: a.token_type,
                                value: a.value,
                            };
                            self.local_memory.insert(b.value, out.clone());
                            self.stack.push(out);
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
                                    bold!(&(0..b.value.len())
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
                        self.local_memory.insert(b.value, out.clone());
                        self.stack.push(out);
                    }
                }
            }
            _ => {}
        }
    }

    fn variable_check_pop(&mut self) -> Option<Token> {
        let first = self.stack.pop();

        if let Some(ref a) = first {
            match self.local_memory.get(&a.value) {
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

    fn asm_li(&mut self, token: Token) {
        self.assembly
            .push(format!("li t{} {}", self.temp_reg_index, token.value));
        self.temp_reg_index += 1;
    }

    fn asm_add(&mut self) {
        self.assembly.push(format!(
            "add t{} t{} t{}",
            self.temp_reg_index - 2,
            self.temp_reg_index - 2,
            self.temp_reg_index - 1
        ));
        self.temp_reg_index -= 1;
    }

    fn asm_sub(&mut self) {
        self.assembly.push(format!(
            "sub t{} t{} t{}",
            self.temp_reg_index - 2,
            self.temp_reg_index - 2,
            self.temp_reg_index - 1
        ));
        self.temp_reg_index -= 1;
    }

    fn match_token_type(&mut self, token: Token) {
        // Match the type of token
        // if it's a literal, add it to the stack
        // if it's an operation, pop values from the stack and apply the operation
        match token.token_type {
            TokenType::Greater => {
                self.function_mode = true;
            }

            TokenType::NumericIntLiteral => {
                self.asm_li(token.clone());
                self.stack.push(token);
            }

            TokenType::NumericDecLiteral => {
                self.stack.push(token);
            }

            TokenType::BoolLiteral => match token.value.as_str() {
                "true" => self.stack.push(Token {
                    token_type: TokenType::BoolLiteral,
                    value: "1".to_string(),
                }),
                "false" => self.stack.push(Token {
                    token_type: TokenType::BoolLiteral,
                    value: "0".to_string(),
                }),
                _ => {}
            },

            TokenType::Bang => {
                let first = self.variable_check_pop();

                self.convert_to_bool(first, token);
                let second = self.variable_check_pop();

                match second {
                    Some(b) => {
                        let a_float = b
                            .value
                            .parse::<f64>()
                            .expect("Bool should always be readable here");

                        self.stack.push(Token {
                            token_type: TokenType::BoolLiteral,
                            value: (a_float == 0.0).to_string(),
                        });
                    }
                    None => stack_empty_error(),
                }
            }

            TokenType::Question => {
                let three = self.variable_check_pop();
                let two = self.variable_check_pop();
                let one = self.variable_check_pop();

                match three {
                    Some(a) => {
                        if a.value == "1" {
                            self.assign_value(one, two, token)
                        } else {
                            // Don't assign
                        }
                    }
                    None => stack_empty_error(),
                }
            }

            TokenType::Identifier => {
                let var = self.local_memory.get(&token.value);
                let func = self.function_memory.get(&token.value);

                match (var, func) {
                    // Push the value the variable is associated with
                    (Some(tok), None) => {
                        if self.stack.len() == 0 {
                            self.stack.push(tok.clone())
                        } else {
                            self.stack.push(token);
                        }
                    }
                    // Push the token of type Identifier as an Identifier to the stack
                    (None, None) => self.stack.push(token),
                    (None, Some(f)) => {
                        self.token_stack.append(&mut f.clone());
                    }
                    (_, _) => {
                        unreachable!();
                    }
                }
            }

            TokenType::TypeSqrtKeyword => {
                let first = self.variable_check_pop();

                if let Some(a) = first {
                    let a_float_res = a.value.parse::<f64>();
                    match a_float_res {
                        Ok(a_val) => self.stack.push(Token {
                            token_type: TokenType::NumericIntLiteral,
                            value: ((a_val as f64).sqrt()).to_string(),
                        }),
                        Err(_) => wrong_type_error_first(a.value, token.value),
                    }
                } else {
                    stack_empty_error();
                }
            }

            // Create variables
            TokenType::Assignment => {
                // Use raw stack.pop here
                let second = self.stack.pop();
                let first = self.stack.pop();

                self.assign_value(first, second, token);
            }

            TokenType::TypeIntKeyword => {
                let first = self.variable_check_pop();

                match first {
                    Some(a) => match a.token_type {
                        TokenType::NumericIntLiteral
                        | TokenType::NumericDecLiteral
                        | TokenType::BoolLiteral => {
                            let a_float_res = a.value.parse::<f64>();
                            match a_float_res {
                                Ok(a_val) => self.stack.push(Token {
                                    token_type: TokenType::NumericIntLiteral,
                                    value: (a_val as i64).to_string(),
                                }),
                                Err(_) => wrong_type_error_first(a.value, token.value),
                            }
                        }

                        _ => invalid_type_cast_error(String::from("NumericIntLiteral"), a, token),
                    },
                    None => stack_empty_error(),
                }
            }

            TokenType::TypeDecKeyword => {
                let first = self.variable_check_pop();

                match first {
                    Some(a) => match a.token_type {
                        TokenType::NumericIntLiteral
                        | TokenType::NumericDecLiteral
                        | TokenType::BoolLiteral => {
                            let a_float_res = a.value.parse::<f64>();
                            match a_float_res {
                                Ok(a_val) => self.stack.push(Token {
                                    token_type: TokenType::NumericDecLiteral,
                                    value: (a_val as f64).to_string(),
                                }),
                                Err(_) => wrong_type_error_first(a.value, token.value),
                            }
                        }
                        _ => invalid_type_cast_error(String::from("NumericIntLiteral"), a, token),
                    },
                    None => stack_empty_error(),
                }
            }

            TokenType::TypeBoolKeyword => {
                let first = self.variable_check_pop();

                self.convert_to_bool(first, token);
            }

            // Simple two argument operations/functions
            TokenType::Addition
            | TokenType::Multiplication
            | TokenType::Subtraction
            | TokenType::Division
            | TokenType::Carrot => {
                let second = self.variable_check_pop();
                let first = self.variable_check_pop();

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
                                            self.stack.push(Token {
                                                token_type: t_type,
                                                value: v.to_string(),
                                            });
                                            self.asm_add();
                                        }
                                        TokenType::Multiplication => {
                                            let v = a_float * b_float;
                                            let t_type = if v.fract() == 0.0 && keep_int {
                                                TokenType::NumericIntLiteral
                                            } else {
                                                TokenType::NumericDecLiteral
                                            };
                                            self.stack.push(Token {
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
                                            self.stack.push(Token {
                                                token_type: t_type,
                                                value: v.to_string(),
                                            });
                                            self.asm_sub();
                                        }
                                        TokenType::Division => {
                                            let v = a_float / b_float;
                                            let t_type = if v.fract() == 0.0 && keep_int {
                                                TokenType::NumericIntLiteral
                                            } else {
                                                TokenType::NumericDecLiteral
                                            };
                                            self.stack.push(Token {
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
                                            self.stack.push(Token {
                                                token_type: t_type,
                                                value: v.to_string(),
                                            });
                                        }
                                        _ => {
                                            unreachable!();
                                        }
                                    },

                                    // Give errors if values did not parse correctly
                                    (Err(_), Ok(_)) => wrong_type_error_first(b.value, token.value),
                                    (Ok(_), Err(_)) => {
                                        wrong_type_error_second(a.value, b.value, token.value)
                                    }
                                    (Err(_), Err(_)) => {
                                        wrong_type_error_both(a.value, b.value, token.value)
                                    }
                                }
                            }

                            // Give errors if values are not NumericIntLiteral or NumericDecLiteral
                            (_, TokenType::NumericIntLiteral) => {
                                wrong_type_error_first(b.value, token.value)
                            }

                            (TokenType::NumericIntLiteral, _) => {
                                wrong_type_error_first(a.value, token.value)
                            }
                            (_, _) => wrong_type_error_both(a.value, b.value, token.value),
                        }
                    }
                    _ => stack_empty_error(),
                }
            }

            _ => {
                println!(
                    "{} Operation Not Implemented [E5]",
                    color!(Color::RED, bold!("Error:").as_str()).as_str()
                );
                println!(
                    "{}",
                    color!(Color::RED, bold!(token.value.as_str()).as_str()),
                );
                println!(
                    "{} operation is not implemented",
                    color!(
                        Color::RED,
                        bold!(&(0..token.value.len()).map(|_| "^").collect::<String>()).as_str()
                    ),
                );
            }
        }
    }

    fn parse(&mut self, tokens: Vec<Token>) -> Token {
        self.reset_asm();

        if self.verbose {
            println!(
                "{}",
                color!(
                    Color::BLACK,
                    format!("Stack before: {:?}", self.stack).as_str()
                )
            );
        }

        self.function_mode = false;
        self.token_stack.append(&mut tokens.clone());

        // Parse postfix notation
        while self.token_index < self.token_stack.len() {
            let token = self.token_stack[self.token_index].clone();

            if self.verbose {
                println!(
                    "{}",
                    color!(Color::BLACK, format!("Reading token: {:?}", token).as_str())
                );
                println!(
                    "{}",
                    color!(Color::BLACK, format!("Stack: {:?}", self.stack).as_str())
                );
                println!(
                    "{}",
                    color!(
                        Color::BLACK,
                        format!("Function memory: {:?}", self.function_memory).as_str()
                    )
                );
            }

            if self.function_mode {
                match token.token_type {
                    TokenType::Function => {
                        self.function_mode = false;

                        let func_name = self.function_stack.pop();

                        if let Some(name) = func_name {
                            match self.function_memory.get(&name.value) {
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
                                    self.function_memory
                                        .insert(name.value, self.function_stack.clone());
                                    // Clear the function off the stack
                                    self.function_stack = Vec::<Token>::new();

                                    if self.verbose {
                                        println!(
                                            "{}",
                                            color!(
                                                Color::BLACK,
                                                format!(
                                                    "Function memory: {:?}",
                                                    self.function_memory
                                                )
                                                .as_str()
                                            )
                                        );
                                    }
                                }
                            }
                        }
                        self.token_index += 1;

                        continue;
                    }
                    _ => {
                        self.function_stack.push(token.clone());
                    }
                }
            } else {
                self.match_token_type(token);
            }

            self.token_index += 1;
        }

        if self.verbose {
            println!(
                "{}",
                color!(
                    Color::BLACK,
                    format!("Stack after: {:?}", self.stack).as_str()
                )
            );
        }

        match self.stack.pop() {
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
}

pub fn create_parser(verbose: bool) -> ParserState {
    let mut local_memory = HashMap::new();
    let function_memory = HashMap::new();

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

    ParserState {
        function_mode: false,
        verbose,
        function_memory,
        local_memory,
        stack: Vec::<Token>::new(),
        token_stack: Vec::<Token>::new(),
        function_stack: Vec::<Token>::new(),
        token_index: 0,
        assembly: Vec::<String>::new(),
        temp_reg_index: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test_1() {
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

        let mut p = create_parser(true);
        let out1 = p.parse(input1);

        assert_eq!(
            out1,
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "2".to_string(),
            },
        );
    }

    #[test]
    fn parse_test_2() {
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

        let mut p = create_parser(true);
        let out2 = p.parse(input2);

        assert_eq!(
            out2,
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "6".to_string(),
            },
        );
    }

    #[test]
    fn parse_test_3() {
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

        let mut p = create_parser(true);
        let out3 = p.parse(input3);

        assert_eq!(
            out3,
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "50".to_string(),
            },
        );
    }

    #[test]
    fn parse_test_4() {
        let input4 = vec![
            Token {
                token_type: TokenType::Greater,
                value: ">".to_string(),
            },
            Token {
                token_type: TokenType::Identifier,
                value: "t".to_string(),
            },
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "1".to_string(),
            },
            Token {
                token_type: TokenType::Identifier,
                value: "v".to_string(),
            },
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "2".to_string(),
            },
            Token {
                token_type: TokenType::Carrot,
                value: "^".to_string(),
            },
            Token {
                token_type: TokenType::Identifier,
                value: "C".to_string(),
            },
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "2".to_string(),
            },
            Token {
                token_type: TokenType::Carrot,
                value: "^".to_string(),
            },
            Token {
                token_type: TokenType::Division,
                value: "/".to_string(),
            },
            Token {
                token_type: TokenType::Subtraction,
                value: "-".to_string(),
            },
            Token {
                token_type: TokenType::TypeSqrtKeyword,
                value: "sqrt".to_string(),
            },
            Token {
                token_type: TokenType::Division,
                value: "/".to_string(),
            },
            Token {
                token_type: TokenType::Identifier,
                value: "s".to_string(),
            },
            Token {
                token_type: TokenType::Function,
                value: "fn".to_string(),
            },
        ];

        let mut p = create_parser(true);
        let out4 = p.parse(input4);

        assert_eq!(
            out4,
            Token {
                token_type: TokenType::NoType,
                value: "".to_string(),
            },
        );

        let input5 = vec![
            // Set var v
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "300040".to_string(),
            },
            Token {
                token_type: TokenType::Identifier,
                value: "v".to_string(),
            },
            Token {
                token_type: TokenType::Assignment,
                value: "=".to_string(),
            },
            // Set var t
            Token {
                token_type: TokenType::NumericIntLiteral,
                value: "30".to_string(),
            },
            Token {
                token_type: TokenType::Identifier,
                value: "t".to_string(),
            },
            Token {
                token_type: TokenType::Assignment,
                value: "=".to_string(),
            },
            // Call s
            Token {
                token_type: TokenType::Identifier,
                value: "s".to_string(),
            },
        ];

        let out5 = p.parse(input5);

        assert_eq!(
            out5,
            Token {
                token_type: TokenType::NumericDecLiteral,
                value: "30.00001502479285".to_string(),
            },
        );
    }
}

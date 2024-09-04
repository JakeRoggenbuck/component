use itertools::Itertools;

fn is_char_symbol(ch: char) -> bool {
    match ch {
        '[' | ']' | '{' | '}' | '(' | ')' | '.' | ',' | ':' | ';' | '=' | '\'' | '\"' | '\\'
        | '+' | '-' | '*' | '/' | '^' | '>' | '<' | '~' => true,
        _ => false,
    }
}

fn is_non_zero_number(ch: char) -> bool {
    match ch {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn is_number(ch: char) -> bool {
    ch == '0' || is_non_zero_number(ch)
}

fn is_char_whitespace(ch: char) -> bool {
    match ch {
        '\t' | ' ' | '\n' => true,
        _ => false,
    }
}

fn is_part_whitespace(string: &str) -> bool {
    for s in string.chars() {
        if !is_char_whitespace(s) {
            return false;
        }
    }

    true
}

fn ends_token(cur: char, next: char) -> bool {
    if is_char_whitespace(next) {
        return true;
    }

    if is_char_symbol(cur) || is_char_symbol(next) {
        return true;
    }

    if is_char_whitespace(cur) {}
    return false;
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    NoType = 0,

    // 100, 5, 34298, -43
    NumericIntLiteral = 1,

    // 100.10, 0.0124, 45.453
    // NOT: .3, 54
    NumericDecLiteral = 2,

    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,

    Dot,
    Comma,

    Addition,
    Subtraction,
    Multiplication,
    Division,

    Greater,
    Less,

    Assignment,
    Semicolon,
    Colon,
    Tag,
    Reference,
    Question,
    At,
    Percent,
    Bang,
    BackSlash,

    Space,
    Tab,
    Newline,

    SingleQuote,
    DoubleQuote,

    TypeNumberKeyword,
    // "int"
    TypeIntKeyword,
    TypeRatioKeyword,
    TypeRealKeyword,
    TypeDecKeyword,
    TypeComplexKeyword,
    TypeImaginaryKeyword,
    TypeSizeKeyword,

    // Variable name like "a"
    Identifier,

    EndToken,
}

fn is_type(maybe_type: &str) -> TokenType {
    match maybe_type {
        // Number types:
        "number" => TokenType::TypeNumberKeyword,
        "int" => TokenType::TypeIntKeyword,
        "ratio" => TokenType::TypeRatioKeyword,
        "real" => TokenType::TypeRealKeyword,
        "dec" => TokenType::TypeDecKeyword,
        "complex" => TokenType::TypeComplexKeyword,
        "imaginary" => TokenType::TypeImaginaryKeyword,
        "size" => TokenType::TypeSizeKeyword,

        // Other types
        // "literal" | "type" | "option" | "string" => true,
        _ => TokenType::NoType,
    }
}

pub trait TokenTrait {
    fn default() -> Self;
    fn tokenize(tokens: String) -> Self;
    fn from_chars(chars: Vec<char>) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
// TODO: Change value: String to value: TokenValue and make TokenValue an enum with many types
// supported like int, float, string, etc.
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

fn is_part_int_numeric(part: &str) -> bool {
    let mut chars = part.chars();

    // Literal 0 case
    let mut a = chars.clone();
    if a.clone().count() == 1 && a.nth(0) == Some('0') {
        return true;
    }

    let first_char = chars
        .nth(0)
        .expect("Part should have more than zero characters");
    if !(is_non_zero_number(first_char) || first_char == '-') {
        return false;
    }

    for c in chars {
        if !is_number(c) {
            return false;
        }
    }

    return true;
}

fn is_part_alpha(part: &str) -> bool {
    let chars = part.chars();

    for c in chars {
        if !c.is_alphabetic() {
            return false;
        }
    }

    return true;
}

impl TokenTrait for Token {
    fn default() -> Self {
        return Token {
            token_type: TokenType::NoType,
            value: String::new(),
        };
    }

    fn tokenize(tokens: String) -> Self {
        let token_str = tokens.as_str();
        let mut token = Token::default();

        if tokens.len() == 1 {
            let token_type = match token_str {
                "{" => TokenType::LeftBrace,
                "}" => TokenType::RightBrace,
                "[" => TokenType::LeftBracket,
                "]" => TokenType::RightBracket,
                "(" => TokenType::LeftParen,
                ")" => TokenType::RightParen,

                "." => TokenType::Dot,
                "," => TokenType::Comma,

                "+" => TokenType::Addition,
                "-" => TokenType::Subtraction,
                "*" => TokenType::Multiplication,
                "/" => TokenType::Division,

                ">" => TokenType::Greater,
                "<" => TokenType::Less,

                "=" => TokenType::Assignment,
                ";" => TokenType::Semicolon,
                ":" => TokenType::Colon,
                "#" => TokenType::Tag,
                "&" => TokenType::Reference,
                "?" => TokenType::Question,
                "@" => TokenType::At,
                "%" => TokenType::Percent,
                "!" => TokenType::Bang,
                "\\" => TokenType::BackSlash,

                " " => TokenType::Space,
                "\t" => TokenType::Tab,
                "\n" => TokenType::Newline,

                "\'" => TokenType::SingleQuote,
                "\"" => TokenType::DoubleQuote,

                _ => TokenType::NoType,
            };

            if token_type != TokenType::NoType {
                token.token_type = token_type;
                token.value = tokens;
                return token;
            }
        }

        if is_part_int_numeric(token_str) {
            token.token_type = TokenType::NumericIntLiteral;
            token.value = tokens;
            return token;
        }

        // Check for type keywords like "int" and "dec"
        let tok = is_type(token_str);
        if tok != TokenType::NoType {
            token.token_type = tok;
            token.value = tokens;
            return token;
        }

        // Check for identifiers that are not keywords
        if is_part_alpha(token_str) {
            token.token_type = TokenType::Identifier;
            token.value = tokens;
            return token;
        }

        token.value = tokens;
        return token;
    }

    fn from_chars(chars: Vec<char>) -> Self {
        let string: String = String::from_iter(chars);
        Token::tokenize(string)
    }
}

pub struct Lexer {
    line_index: usize,
    column_index: usize,
    lines: Vec<String>,
    prev_token: Token,
}

pub trait Lex {
    fn new(lines: Vec<String>) -> Self;
    fn next(&mut self) -> Token;

    /// TODO: Get this to move back one token and return it
    fn prev(&mut self) -> Token;

    /// TODO: Get this to show the next token without moving the column_index
    fn peak(&mut self) -> Token;
    /// TODO: Get this to show the previous token without moving the column_index
    fn lookback(&mut self) -> Token;

    fn reset_line(&mut self);
}

impl Lex for Lexer {
    fn new(lines: Vec<String>) -> Self {
        Lexer {
            line_index: 0,
            column_index: 0,
            lines,
            prev_token: Token::default(),
        }
    }

    fn prev(&mut self) -> Token {
        return self.prev_token.clone();
    }

    fn next(&mut self) -> Token {
        let mut buffer = Vec::<char>::new();

        // Note: the last character in the line will never be set to `cur`, thus will never get
        // pushed. You could push(' ') to the line to fix this as seen below. The line is mutable
        // anyway so there isn't much of a real disadvantage do doing this
        self.lines[self.line_index].push(' ');

        let current_line = &self.lines[self.line_index][self.column_index..];

        if is_part_whitespace(current_line) {
            let mut token = Token::default();
            token.token_type = TokenType::EndToken;
            return token;
        }

        // Iterate through using windows of size 2
        // abcd -> (a, b), (b, c), (c, d)
        for (cur, next) in current_line.chars().into_iter().tuple_windows() {
            // Skip whitespace at the start of a new section on un-lexed line
            if is_char_whitespace(cur) {
                self.column_index += 1;
                continue;
            }

            self.column_index += 1;
            buffer.push(cur);
            if ends_token(cur, next) {
                break;
            }
        }

        let new_token = Token::from_chars(buffer);
        return new_token;
    }

    fn reset_line(&mut self) {
        self.column_index = 0;
    }

    /// TODO: Get this to show the next token without moving the column_index
    fn peak(&mut self) -> Token {
        Token::default()
    }

    /// TODO: Get this to show the previous token without moving the column_index
    fn lookback(&mut self) -> Token {
        Token::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_default_test() {
        assert_eq!(Token::default().token_type, TokenType::NoType);
    }

    #[test]
    fn tokenize_test() {
        assert_eq!(
            Token::tokenize("1".to_string()).token_type,
            TokenType::NumericIntLiteral
        );

        assert_eq!(Token::tokenize("1".to_string()).value, "1".to_string());

        // Note: "1 " should NOT be a valid token, because the value given to tokenize should cut
        // off after the 1 because of the ending token check with ends_token
        // However, when this is given to the lexer, it should lex "1 " as a valid
        // NumericIntLiteral type with a value of "1"
        assert_ne!(
            Token::tokenize("1 ".to_string()).token_type,
            TokenType::NumericIntLiteral
        );
        assert_ne!(Token::tokenize("1 ".to_string()).value, "1".to_string());
    }

    #[test]
    fn lexer_test() {
        let mut lex: Lexer = Lexer::new(vec![]);

        lex.lines = vec!["1 2 *".to_string()];

        assert_eq!(lex.next().value, "1");
        assert_eq!(lex.next().value, "2");
        assert_eq!(lex.next().value, "*");

        lex.reset_line();

        assert_eq!(lex.next().token_type, TokenType::NumericIntLiteral);
        assert_eq!(lex.next().token_type, TokenType::NumericIntLiteral);
        assert_eq!(lex.next().token_type, TokenType::Multiplication);

        lex.reset_line();

        lex.lines = vec!["1 1 +".to_string()];

        assert_eq!(lex.next().value, "1");
        assert_eq!(lex.next().value, "1");
        assert_eq!(lex.next().value, "+");

        lex.reset_line();

        assert_eq!(lex.next().token_type, TokenType::NumericIntLiteral);
        assert_eq!(lex.next().token_type, TokenType::NumericIntLiteral);
        assert_eq!(lex.next().token_type, TokenType::Addition);

        lex.reset_line();

        lex.lines = vec!["2 5 /".to_string()];

        assert_eq!(lex.next().value, "2");
        assert_eq!(lex.next().value, "5");
        assert_eq!(lex.next().value, "/");

        lex.reset_line();

        assert_eq!(lex.next().token_type, TokenType::NumericIntLiteral);
        assert_eq!(lex.next().token_type, TokenType::NumericIntLiteral);
        assert_eq!(lex.next().token_type, TokenType::Division);
    }

    #[test]
    fn is_char_symbol_test() {
        assert!(is_char_symbol('+'));
    }

    #[test]
    fn is_char_whitespace_test() {
        assert!(is_char_whitespace(' '));
    }

    #[test]
    fn is_part_int_numeric_test() {
        assert!(is_part_int_numeric("1"));
        assert!(is_part_int_numeric("0"));
    }

    #[test]
    fn ends_token_test() {
        // "1 " is the `1` token
        assert!(ends_token('1', ' '));

        // "+\n" is the `+` token
        assert!(ends_token('+', '\n'));

        // "myvar " is the `literal` token containing "myvar"
        assert!(ends_token('r', ' '));

        // "+=" is the `sum` token
        assert!(ends_token('+', '='));

        // "ab" is not the end of any token and may continue as a literal
        assert!(!ends_token('a', 'b'));
    }
}

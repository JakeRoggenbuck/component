use super::lexer::Token;
use efcl::{bold, color, Color};

pub fn invalid_type_cast_error(cast_to_type: String, one: Token, two: Token) {
    println!(
        "{} Invalid Type Cast [E3]",
        color!(Color::RED, bold!("Error:").as_str()).as_str()
    );
    println!(
        "{} {}",
        one.value,
        color!(Color::RED, bold!(&two.value).as_str())
    );

    println!(
        "{}{} Cannot convert <{}> to <{}>",
        (0..one.value.len() + 1).map(|_| " ").collect::<String>(),
        color!(Color::RED, bold!("^^^").as_str()),
        format!("{:?}", one.token_type),
        cast_to_type
    )
}

pub fn stack_empty_error() {
    println!(
        "{} Stack Empty [E4]",
        color!(Color::RED, bold!("Error:").as_str()).as_str()
    );
}

pub fn wrong_type_error_first(val_one: String, val_two: String) {
    println!(
        "{} Wrong Type [E2]",
        color!(Color::RED, bold!("Error:").as_str()).as_str()
    );
    println!(
        "{} {}",
        color!(Color::RED, bold!(val_one.as_str()).as_str()),
        val_two
    );
    println!(
        "{} value is not a <NumericIntLiteral> or <NumericDecLiteral>",
        color!(Color::RED, bold!("^").as_str())
    );
}

pub fn wrong_type_error_second(val_one: String, val_two: String, val_three: String) {
    println!(
        "{} Wrong Type [E2]",
        color!(Color::RED, bold!("Error:").as_str()).as_str()
    );
    println!(
        "{} {} {}",
        val_one,
        color!(Color::RED, bold!(val_two.as_str()).as_str()),
        val_three
    );
    println!(
        "{}{}value is not a <NumericIntLiteral> or <NumericDecLiteral> because it did not parse correctly",
        (0..val_one.len() + 1).map(|_| " ").collect::<String>(),
        color!(Color::RED, bold!("^").as_str()),
    );
}

pub fn wrong_type_error_both(val_one: String, val_two: String, val_three: String) {
    println!(
        "{} Wrong Type [E2]",
        color!(Color::RED, bold!("Error:").as_str()).as_str()
    );
    println!(
        "{} {} {}",
        color!(Color::RED, bold!(val_one.as_str()).as_str()),
        color!(Color::RED, bold!(val_two.as_str()).as_str()),
        val_three
    );
    println!(
        "{}{}{} values are not a <NumericIntLiteral> or <NumericDecLiteral> because they did not parse correctly",
        color!(Color::RED, bold!("^").as_str()),
        (0..val_one.len()).map(|_| " ").collect::<String>(),
        color!(Color::RED, bold!("^").as_str())
    );
}

use logical_expression_parser::parser::{ParserError, parse};

fn main() -> Result<(), ParserError> {
    let input = "A OR B";

    let expression_str = parse(input)?
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .as_str();

    println!("{}", expression_str);

    Ok(())
}

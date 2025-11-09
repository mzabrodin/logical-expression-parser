use anyhow::{Result, anyhow};
use logical_expression_parser::parse;

#[test]
fn test_unexpected_command() -> Result<()> {
    let input = "(A OR B) And C";
    let file_pair = parse(input);
    assert!(file_pair.is_err());

    Ok(())
}

#[test]
fn test_unexpected_identifier() -> Result<()> {
    let input = "A OR B1";
    let file_pair = parse(input);
    assert!(file_pair.is_err());

    Ok(())
}

#[test]
fn test_unclosed_parenthesis() -> Result<()> {
    let input = "!A OR (B OR !C";
    let file_pair = parse(input);
    assert!(file_pair.is_err());

    Ok(())
}

#[test]
fn test_nested_parentheses() -> Result<()> {
    let input = "(A OR (B or !C))";
    let file_pair = parse(input)?
        .next()
        .ok_or_else(|| anyhow!("No file pair"))?;
    let expression_pair = file_pair
        .into_inner()
        .next()
        .ok_or_else(|| anyhow!("No expression"))?;
    assert_eq!(expression_pair.as_str(), input);

    Ok(())
}

#[test]
fn test_simple_or() -> Result<()> {
    let input = "A OR B";
    let file_pair = parse(input)?
        .next()
        .ok_or_else(|| anyhow!("No file pair"))?;
    let expression_pair = file_pair
        .into_inner()
        .next()
        .ok_or_else(|| anyhow!("No expression"))?;
    assert_eq!(
        expression_pair.as_rule(),
        logical_expression_parser::Rule::expression
    );
    assert_eq!(expression_pair.as_str(), input);

    Ok(())
}

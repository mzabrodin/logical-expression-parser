use anyhow::Result;
use logical_expression_parser::parser::{Grammar, Rule};
use pest::Parser;

#[test]
fn test_identifier_valid() -> Result<()> {
    let pairs = Grammar::parse(Rule::identifier, "A")?;
    assert_eq!(pairs.as_str(), "A");

    Ok(())
}

#[test]
fn test_identifier_invalid_lowercase() -> Result<()> {
    let pairs = Grammar::parse(Rule::identifier, "a");
    assert!(pairs.is_err());

    Ok(())
}

#[test]
fn test_identifier_only_single_character() -> Result<()> {
    let pairs = Grammar::parse(Rule::identifier, "VAR");
    assert!(pairs.is_err());

    Ok(())
}

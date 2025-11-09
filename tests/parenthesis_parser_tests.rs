use anyhow::Result;
use logical_expression_parser::parser::{Grammar, Rule};
use pest::Parser;

fn test_parenthesis_valid(rule: Rule, symbol: &str) -> Result<()> {
    let pair = Grammar::parse(rule, symbol)?.next().unwrap();
    assert_eq!(pair.as_rule(), rule);
    assert_eq!(pair.as_str(), symbol);

    Ok(())
}

fn test_parenthesis_rule_invalid(rule: Rule, invalid_symbols: &[&str]) -> Result<()> {
    for s in invalid_symbols {
        let result = Grammar::parse(rule, s);
        assert!(result.is_err());
    }

    Ok(())
}

#[test]
fn test_left_parenthesis_valid() -> Result<()> {
    test_parenthesis_valid(Rule::left_parenthesis, "(")
}

#[test]
fn test_right_parenthesis_valid() -> Result<()> {
    test_parenthesis_valid(Rule::right_parenthesis, ")")
}

#[test]
fn test_left_parenthesis_fail() -> Result<()> {
    test_parenthesis_rule_invalid(Rule::left_parenthesis, &["[", "{", "a", ")"])
}

#[test]
fn test_right_parenthesis_fail() -> Result<()> {
    test_parenthesis_rule_invalid(Rule::right_parenthesis, &["]", "}", "b", "("])
}

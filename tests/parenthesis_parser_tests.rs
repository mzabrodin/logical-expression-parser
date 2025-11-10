use anyhow::Result;
use logical_expression_pest_parser::parser::Rule;
mod common;

#[test]
fn test_left_parenthesis_valid() -> Result<()> {
    common::test_if_valid(Rule::left_parenthesis, &["("])
}

#[test]
fn test_right_parenthesis_valid() -> Result<()> {
    common::test_if_valid(Rule::right_parenthesis, &[")"])
}

#[test]
fn test_left_parenthesis_invalid() -> Result<()> {
    common::test_if_invalid(Rule::left_parenthesis, &["[", "{", "a", ")"])
}

#[test]
fn test_right_parenthesis_invalid() -> Result<()> {
    common::test_if_invalid(Rule::right_parenthesis, &["]", "}", "b", "("])
}

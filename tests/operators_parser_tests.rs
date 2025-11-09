use anyhow::Result;
use logical_expression_parser::parser::{Grammar, Rule};
use pest::Parser;

fn test_some_operator_forms_valid(some_operator: Rule, forms: &[&str]) -> Result<()> {
    for operator in forms {
        let pair = Grammar::parse(some_operator, operator)?.next().unwrap();
        assert_eq!(pair.as_rule(), some_operator);
        assert_eq!(pair.as_str(), operator.to_string());
        assert!(pair.clone().into_inner().next().is_none());
    }

    Ok(())
}

fn test_some_operator_forms_invalid(some_operator: Rule, forms: &[&str]) -> Result<()> {
    for operator in forms {
        let result = Grammar::parse(some_operator, operator);
        assert!(result.is_err());
    }

    Ok(())
}

#[test]
fn test_not_operator_forms_valid() -> Result<()> {
    test_some_operator_forms_valid(Rule::not_operator, &["NOT", "not", "!"])
}

#[test]
fn test_and_operator_forms_valid() -> Result<()> {
    test_some_operator_forms_valid(Rule::and_operator, &["AND", "and", "&"])
}

#[test]
fn test_nand_operator_forms_valid() -> Result<()> {
    test_some_operator_forms_valid(Rule::nand_operator, &["NAND", "nand", "!&"])
}

#[test]
fn test_or_operator_forms_valid() -> Result<()> {
    test_some_operator_forms_valid(Rule::or_operator, &["OR", "or", "|"])
}

#[test]
fn test_nor_operator_forms_valid() -> Result<()> {
    test_some_operator_forms_valid(Rule::nor_operator, &["NOR", "nor", "!|"])
}

#[test]
fn test_xor_operator_forms_valid() -> Result<()> {
    test_some_operator_forms_valid(Rule::xor_operator, &["XOR", "xor", "^"])
}

#[test]
fn test_xnor_operator_forms_valid() -> Result<()> {
    test_some_operator_forms_valid(Rule::xnor_operator, &["XNOR", "xnor", "!^"])
}

#[test]
fn test_not_operator_forms_invalid() -> Result<()> {
    test_some_operator_forms_invalid(Rule::not_operator, &["Not", "nOt"])
}

#[test]
fn test_and_operator_forms_invalid() -> Result<()> {
    test_some_operator_forms_invalid(Rule::and_operator, &["And", "aNd", "||"])
}

#[test]
fn test_nand_operator_forms_invalid() -> Result<()> {
    test_some_operator_forms_invalid(Rule::nand_operator, &["Nand", "nAnd", "!|"])
}

#[test]
fn test_or_operator_forms_invalid() -> Result<()> {
    test_some_operator_forms_invalid(Rule::or_operator, &["Or", "o r", "&&"])
}

#[test]
fn test_nor_operator_forms_invalid() -> Result<()> {
    test_some_operator_forms_invalid(Rule::nor_operator, &["Nor", "n O r", "!&"])
}

#[test]
fn test_xor_operator_forms_invalid() -> Result<()> {
    test_some_operator_forms_invalid(Rule::xor_operator, &["Xor", "xOr", "!"])
}

#[test]
fn test_xnor_operator_forms_invalid() -> Result<()> {
    test_some_operator_forms_invalid(Rule::xnor_operator, &["Xnor", "xNoR", "%"])
}
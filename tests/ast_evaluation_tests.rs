use anyhow::Result;
use logical_expression_pest_parser::parse_expression;
use std::collections::HashMap;

fn vars(pairs: &[(char, bool)]) -> HashMap<char, bool> {
    pairs.iter().copied().collect()
}

fn eval(input: &str, pairs: &[(char, bool)]) -> Result<bool> {
    Ok(parse_expression(&format!("{input}\n"))?.evaluate(&vars(pairs)))
}

#[test]
fn test_and_truth_table() -> Result<()> {
    assert!(!eval("A AND B", &[('A', false), ('B', false)])?);
    assert!(!eval("A AND B", &[('A', true), ('B', false)])?);
    assert!(!eval("A AND B", &[('A', false), ('B', true)])?);
    assert!(eval("A AND B", &[('A', true), ('B', true)])?);
    Ok(())
}

#[test]
fn test_nand_truth_table() -> Result<()> {
    assert!(eval("A NAND B", &[('A', false), ('B', false)])?);
    assert!(eval("A NAND B", &[('A', true), ('B', false)])?);
    assert!(!eval("A NAND B", &[('A', true), ('B', true)])?);
    Ok(())
}

#[test]
fn test_or_truth_table() -> Result<()> {
    assert!(!eval("A OR B", &[('A', false), ('B', false)])?);
    assert!(eval("A OR B", &[('A', true), ('B', false)])?);
    assert!(eval("A OR B", &[('A', true), ('B', true)])?);
    Ok(())
}

#[test]
fn test_nor_truth_table() -> Result<()> {
    assert!(eval("A NOR B", &[('A', false), ('B', false)])?);
    assert!(!eval("A NOR B", &[('A', true), ('B', false)])?);
    assert!(!eval("A NOR B", &[('A', true), ('B', true)])?);
    Ok(())
}

#[test]
fn test_xor_truth_table() -> Result<()> {
    assert!(!eval("A XOR B", &[('A', false), ('B', false)])?);
    assert!(eval("A XOR B", &[('A', true), ('B', false)])?);
    assert!(!eval("A XOR B", &[('A', true), ('B', true)])?);
    Ok(())
}

#[test]
fn test_xnor_truth_table() -> Result<()> {
    assert!(eval("A XNOR B", &[('A', false), ('B', false)])?);
    assert!(!eval("A XNOR B", &[('A', true), ('B', false)])?);
    assert!(eval("A XNOR B", &[('A', true), ('B', true)])?);
    Ok(())
}

#[test]
fn test_not() -> Result<()> {
    assert!(eval("!A", &[('A', false)])?);
    assert!(!eval("!A", &[('A', true)])?);
    Ok(())
}

#[test]
fn test_double_negation_cancels() -> Result<()> {
    assert!(eval("!!A", &[('A', true)])?);
    assert!(!eval("!!A", &[('A', false)])?);
    Ok(())
}

#[test]
fn test_or_binds_looser_than_and() -> Result<()> {
    // "A OR B AND C" must parse as "A OR (B AND C)".
    // If it were evaluated left-to-right instead, (A OR B) AND C would be false here.
    assert!(eval(
        "A OR B AND C",
        &[('A', true), ('B', false), ('C', false)]
    )?);
    Ok(())
}

#[test]
fn test_and_binds_looser_than_xor() -> Result<()> {
    // "A AND B XOR C" must parse as "A AND (B XOR C)".
    // If it were evaluated left-to-right instead, (A AND B) XOR C would be true here.
    assert!(!eval(
        "A AND B XOR C",
        &[('A', false), ('B', true), ('C', true)]
    )?);
    Ok(())
}

#[test]
fn test_parentheses_override_precedence() -> Result<()> {
    // Without parentheses "(A OR B) AND C" would be "A OR (B AND C)" = true here.
    assert!(!eval(
        "(A OR B) AND C",
        &[('A', true), ('B', false), ('C', false)]
    )?);
    Ok(())
}

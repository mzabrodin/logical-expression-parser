use anyhow::Result;
use logical_expression_pest_parser::parser::Rule;
mod common;

#[test]
fn test_term_valid() -> Result<()> {
    common::test_if_valid(
        Rule::term,
        &["A", "B", "!A", "NOT A", "not A", "!!A", "(A)", "!(A)"],
    )
}

#[test]
fn test_term_invalid() -> Result<()> {
    common::test_if_invalid(Rule::term, &["", "1", "a", "*"])
}

#[test]
fn test_xor_clause_valid() -> Result<()> {
    common::test_if_valid(
        Rule::xor_clause,
        &["A", "A XOR B", "A XOR !B", "A XNOR B", "A XOR !(A XNOR B)"],
    )
}

#[test]
fn test_xor_clause_invalid() -> Result<()> {
    common::test_if_invalid(
        Rule::xor_clause,
        &["XOR B C", "XOR !B", "XOR !(A XNOR B)", ""],
    )
}

#[test]
fn test_and_clause_valid() -> Result<()> {
    common::test_if_valid(
        Rule::and_clause,
        &[
            "A",
            "A AND B",
            "A NAND B",
            "A AND B AND C",
            "A AND !B",
            "A XOR B AND C",
        ],
    )
}

#[test]
fn test_and_clause_invalid() -> Result<()> {
    common::test_if_invalid(
        Rule::and_clause,
        &["AND A B", "AND B", "AND C", "AND !B", ""],
    )
}

#[test]
fn test_expression_valid() -> Result<()> {
    common::test_if_valid(
        Rule::expression,
        &[
            "A",
            "B",
            "!A",
            "A OR B",
            "A OR !!!(B XOR H AND F)",
            "A NOR B AND C",
            "A XOR F AND !B",
        ],
    )
}

#[test]
fn test_expression_invalid() -> Result<()> {
    common::test_if_invalid(Rule::expression, &["OR A B", "OR", "NOR", "NOR A B", ""])
}

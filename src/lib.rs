#![doc = include_str!("../docs.md")]

/// # Abstract Syntax Tree (AST) Module
///
/// Defines the `Expression` enum, which represents a node in the AST, and implements methods for its creation, evaluation, and variable collection.
pub mod ast;

/// # Parser Module
///
/// Defines the Pest `Grammar`, parser errors, and the `parse` function.
pub mod parser;

/// # Truth Table Module
///
/// Defines `TruthTable` and `TruthTableRow`, implements the `Display` trait for both structs and the `From` trait for `TruthTable`.
pub mod truth_table;

use ast::Expression;
use parser::{ParserError, Rule};

/// Parses a single logical expression string directly into an [Expression] AST.
///
/// This is a convenience wrapper around [`parse_expressions`] for consumers who only
/// need the first expression of the input.
///
/// # Errors
/// Returns a [ParserError] under the same conditions as [`parser::parse`].
pub fn parse_expression(input: &str) -> Result<Expression, ParserError> {
    let (_, expression) = parse_expressions(input)?
        .into_iter()
        .next()
        .expect("file rule always contains at least one expression");

    Ok(expression)
}

/// Parses every expression in a (possibly multi-line) input into `(source_text, Expression)`
/// pairs, one per line, in order.
///
/// This is a convenience wrapper around [`parser::parse`] + [`Expression::ast`] for
/// consumers who don't want to work with Pest's `Pairs`/`Rule` types directly.
///
/// # Errors
/// Returns a [ParserError] under the same conditions as [`parser::parse`].
pub fn parse_expressions(input: &str) -> Result<Vec<(&str, Expression)>, ParserError> {
    let mut pairs = parser::parse(input)?;
    let file_pair = pairs.next().expect("file rule always produces one pair");

    Ok(file_pair
        .into_inner()
        .filter(|pair| pair.as_rule() == Rule::expression)
        .map(|pair| (pair.as_str(), Expression::ast(pair)))
        .collect())
}

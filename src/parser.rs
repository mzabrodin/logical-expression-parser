use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;
use thiserror::Error;

/// The Pest parser struct.
///
/// The grammar is included directly from the `grammar.pest` file.
#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

/// Custom parsing error created using the [thiserror] crate.
#[derive(Debug, Error)]
pub enum ParserError {
    /// Wrapper for a [pest] crate error.
    #[error("ParsingError: {0}")]
    PestError(Box<pest::error::Error<Rule>>),

    /// Error for empty input.
    #[error("Empty input")]
    EmptyInputError,
}

/// Parses an input string into `Pairs<Rule>`.
///
/// It checks for empty error and then uses [Grammar] to parse the input with the `file` rule.
///
/// # Arguments
/// * `input` - The string to parse.
///
/// # Returns
/// A [Result] containing successful `Pairs<'_, Rule>` or a [ParserError].
///
/// # Errors
/// Returns [ParserError::PestError] if Pest fails to parse the input string.
///
/// Returns [ParserError::EmptyInputError] if `input` is empty.
pub fn parse(input: &str) -> Result<Pairs<'_, Rule>, ParserError> {
    if input.is_empty() {
        return Err(ParserError::EmptyInputError);
    }

    let pairs =
        Grammar::parse(Rule::file, input).map_err(|e| ParserError::PestError(Box::new(e)))?;
    Ok(pairs)
}

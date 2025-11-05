use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Parsing failed: {0}")]
    PestError(String),

    #[error("Empty input")]
    EmptyInputError,
}

pub fn parse(input: &str) -> Result<Pairs<'_, Rule>, ParserError> {
    if input.is_empty() {
        return Err(ParserError::EmptyInputError);
    }

    Grammar::parse(Rule::file, input).map_err(|err| ParserError::PestError(err.to_string()))
}

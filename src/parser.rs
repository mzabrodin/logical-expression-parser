use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("ParsingError: {0}")]
    PestError(Box<pest::error::Error<Rule>>),

    #[error("Empty input")]
    EmptyInputError,
}

pub fn parse(input: &str) -> Result<Pairs<'_, Rule>, ParserError> {
    if input.is_empty() {
        return Err(ParserError::EmptyInputError);
    }

    let pairs =
        Grammar::parse(Rule::file, input).map_err(|e| ParserError::PestError(Box::new(e)))?;
    Ok(pairs)
}

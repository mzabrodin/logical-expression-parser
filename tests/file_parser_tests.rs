use anyhow::{Result, anyhow};
use logical_expression_parser::parse;

#[test]
fn test_empty_input() -> Result<()> {
    let input = "";
    let file_pair = parse(input);
    assert!(file_pair.is_err());

    Ok(())
}

#[test]
fn test_file() -> Result<()> {
    let input = "A OR B";
    let file_pair = parse(input)?
        .clone()
        .next()
        .ok_or_else(|| anyhow!("No file pair"))?;

    assert_eq!(file_pair.as_rule(), logical_expression_parser::Rule::file);
    assert_eq!(file_pair.as_str(), input);
    assert_eq!(file_pair.as_span().start(), 0);
    assert_eq!(file_pair.as_span().end(), input.len());

    Ok(())
}

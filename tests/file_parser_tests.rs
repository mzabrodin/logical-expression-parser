use anyhow::Result;
use logical_expression_pest_parser::parser::parse;

#[test]
fn test_empty_input() -> Result<()> {
    let input = "";
    let file_pair = parse(input);
    assert!(file_pair.is_err());

    Ok(())
}

#[test]
fn test_file_valid() -> Result<()> {
    let input = "(X NOR U) OR !(A AND B)\n";
    let result = parse(input)?.clone().next().unwrap();

    assert_eq!(
        result.as_rule(),
        logical_expression_pest_parser::parser::Rule::file
    );
    assert_eq!(result.as_str(), input);

    Ok(())
}

#[test]
fn test_file_invalid() -> Result<()> {
    let input = "(X NOR U) OR !(A B)";
    let result = parse(input);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_unclosed_parenthesis() -> Result<()> {
    let input = "A OR (C XNOR B";
    let result = parse(input);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_invalid_identifier() -> Result<()> {
    let input = "A OR B1";
    let result = parse(input);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_invalid_command() -> Result<()> {
    let input = "A || B";
    let result = parse(input);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_nested_parenthesis() -> Result<()> {
    let input = "A OR (C XNOR (!X OR B) OR C)\n";
    let result = parse(input)?.next().unwrap();
    assert_eq!(result.as_str(), input);

    Ok(())
}

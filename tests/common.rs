use logical_expression_pest_parser::parser::{Grammar, Rule};
use pest::Parser;

pub fn test_if_valid(rule: Rule, data: &[&str]) -> anyhow::Result<()> {
    for some in data {
        let pair = Grammar::parse(rule, some)?.next().unwrap();
        assert_eq!(pair.as_rule(), rule);
        assert_eq!(pair.as_str(), some.to_string());
    }

    Ok(())
}

pub fn test_if_invalid(rule: Rule, data: &[&str]) -> anyhow::Result<()> {
    for some in data {
        let pair = Grammar::parse(rule, some);
        assert!(pair.is_err());
    }

    Ok(())
}

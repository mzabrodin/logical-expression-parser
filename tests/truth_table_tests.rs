use anyhow::Result;
use logical_expression_pest_parser::parse_expression;
use logical_expression_pest_parser::truth_table::TruthTable;

#[test]
fn test_row_count_is_two_pow_n() -> Result<()> {
    let expr = parse_expression("A AND B AND C\n")?;
    let table = TruthTable::from(&expr);
    assert_eq!(table.rows.len(), 8);
    Ok(())
}

#[test]
fn test_variables_are_sorted_and_deduplicated() -> Result<()> {
    let expr = parse_expression("C AND A AND B AND A\n")?;
    let table = TruthTable::from(&expr);
    assert_eq!(table.variables, vec!['A', 'B', 'C']);
    Ok(())
}

#[test]
fn test_and_truth_table_rows() -> Result<()> {
    let expr = parse_expression("A AND B\n")?;
    let table = TruthTable::from(&expr);

    assert_eq!(table.variables, vec!['A', 'B']);
    assert_eq!(table.rows.len(), 4);

    let only_true_row = table
        .rows
        .iter()
        .find(|row| row.result)
        .expect("A AND B has exactly one true row");
    assert_eq!(only_true_row.values, vec![true, true]);

    let false_rows_count = table.rows.iter().filter(|row| !row.result).count();
    assert_eq!(false_rows_count, 3);
    Ok(())
}

#[test]
fn test_display_renders_header_and_rows() -> Result<()> {
    let expr = parse_expression("A AND B\n")?;
    let table = TruthTable::from(&expr);
    let rendered = table.to_string();

    assert!(rendered.contains('A'));
    assert!(rendered.contains('B'));
    assert!(rendered.contains("Output"));
    // header + separator + one line per row
    assert_eq!(rendered.lines().count(), table.rows.len() + 2);
    Ok(())
}

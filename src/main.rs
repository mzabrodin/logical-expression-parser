use logical_expression_parser::ast::Expression;
use logical_expression_parser::parser::{ParserError, parse};
use logical_expression_parser::truth_table::{TruthTable, TruthTableRow};

fn main() -> Result<(), ParserError> {
    let input = "(A & B) | C";

    let expression_str = parse(input)?
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .as_str();

    println!("{:#?}", expression_str);

    let pair = parse(input)?.next().unwrap();
    let ast = Expression::ast(pair);

    println!("{:#?}", ast);

    let table = TruthTable {
        variables: vec!['A', 'B'],
        rows: vec![
            TruthTableRow {
                values: vec![false, false],
                result: false,
            },
            TruthTableRow {
                values: vec![true, false],
                result: true,
            },
        ],
    };

    println!("{}", table);

    Ok(())
}

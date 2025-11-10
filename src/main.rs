use logical_expression_parser::ast::Expression;
use logical_expression_parser::parser::{ParserError, parse};
use logical_expression_parser::truth_table::TruthTable;

fn main() -> Result<(), ParserError> {
    let input = "(A NOR B) & C";

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

    let truth_table = TruthTable::from(&ast);
    println!("{}", truth_table);

    Ok(())
}

use crate::ast::Expression;
use comfy_table::Table;
use comfy_table::presets::ASCII_MARKDOWN;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// Represents a single row in the truth table, containing variable values and the evaluated result.
#[derive(Debug)]
pub struct TruthTableRow {
    /// The specific boolean values for each variable in this row
    pub values: Vec<bool>,
    /// The evaluated boolean result of the expression for this row.
    pub result: bool,
}

/// Represents the complete truth table for a given expression, with all possible value combinations and their corresponding results.
#[derive(Debug)]
pub struct TruthTable {
    /// A sorted list of unique variables.
    pub variables: Vec<char>,
    /// A `Vec` of all `TruthTableRow`s, representing the table's body.
    pub rows: Vec<TruthTableRow>,
}

fn bool_cell(value: bool) -> &'static str {
    if value { "1" } else { "0" }
}

impl Display for TruthTable {
    /// Formats a [TruthTable] for printing as a markdown-style table.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.load_preset(ASCII_MARKDOWN);

        let mut header: Vec<String> = self.variables.iter().map(|v| v.to_string()).collect();
        header.push("Output".to_string());
        table.set_header(header);

        for row in &self.rows {
            let mut cells: Vec<&str> = row.values.iter().map(|&v| bool_cell(v)).collect();
            cells.push(bool_cell(row.result));
            table.add_row(cells);
        }

        write!(f, "{table}")
    }
}
impl From<&Expression> for TruthTable {
    /// Creates a `TruthTable`.
    ///
    /// It generates a truth table for a given logical expression showing the evaluation result for all possible combinations of its input variables.
    ///
    /// # Arguments
    /// * `expression` - An AST node that represents a parsed logical expression.
    fn from(expression: &Expression) -> Self {
        let variables = expression.variables();

        let variables_length = variables.len();
        let rows_length = if variables_length == 0 {
            0
        } else {
            1_usize << variables_length
        };

        let mut rows: Vec<TruthTableRow> = Vec::with_capacity(rows_length);
        for row_index in 0..rows_length {
            let mut values = Vec::with_capacity(variables_length);
            let mut idens_values = HashMap::with_capacity(variables_length);

            for (identifier_index, &identifier) in variables.iter().enumerate() {
                let value = (row_index >> identifier_index) & 1 == 1;
                values.push(value);
                idens_values.insert(identifier, value);
            }

            rows.push(TruthTableRow {
                values,
                result: expression.evaluate(&idens_values),
            });
        }

        Self { variables, rows }
    }
}

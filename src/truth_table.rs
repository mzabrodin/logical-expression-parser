use std::fmt::{Display, Formatter};
#[derive(Debug)]
pub struct TruthTableRow {
    pub values: Vec<bool>,
    pub result: bool,
}
#[derive(Debug)]
pub struct TruthTable {
    pub variables: Vec<char>,
    pub rows: Vec<TruthTableRow>,
}

impl Display for TruthTableRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "|")?;
        for &value in &self.values {
            write!(f, " {} |", if value { '1' } else { '0' })?;
        }
        write!(f, "   {}    |", if self.result { '1' } else { '0' })?;

        Ok(())
    }
}

impl Display for TruthTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "|")?;
        for variable in &self.variables {
            write!(f, " {} |", variable)?;
        }
        writeln!(f, " Output |")?;

        write!(f, "|")?;
        for _ in &self.variables {
            write!(f, "---|")?;
        }
        writeln!(f, "--------|")?;

        for row in &self.rows {
            writeln!(f, "{}", row)?;
        }

        Ok(())
    }
}

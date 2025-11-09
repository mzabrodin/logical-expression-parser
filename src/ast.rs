use std::collections::HashMap;

#[derive(Debug)]
pub enum Expression {
    Identifier(char),
    Not(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Nand(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Nor(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
    Xnor(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn evaluate(&self, variables: &HashMap<char, bool>) -> bool {
        match self {
            Expression::Identifier(iden) => variables.get(iden).cloned().unwrap_or(false),
            Expression::Not(expr) => !expr.evaluate(variables),
            Expression::And(left, right) => left.evaluate(variables) && right.evaluate(variables),
            Expression::Nand(left, right) => {
                !(left.evaluate(variables) && right.evaluate(variables))
            }
            Expression::Or(left, right) => left.evaluate(variables) || right.evaluate(variables),
            Expression::Nor(left, right) => {
                !(left.evaluate(variables) || right.evaluate(variables))
            }
            Expression::Xor(left, right) => left.evaluate(variables) ^ right.evaluate(variables),
            Expression::Xnor(left, right) => {
                !(left.evaluate(variables) ^ right.evaluate(variables))
            }
        }
    }
}

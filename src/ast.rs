use crate::parser::Rule;
use pest::iterators::Pair;
use std::collections::{HashMap, HashSet};

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

    pub fn ast(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::identifier => Self::Identifier(pair.as_str().chars().next().unwrap()),

            Rule::term => {
                let mut inner = pair.into_inner();
                let mut not_count = 0;

                let mut next_pair = inner.next().unwrap();
                while next_pair.as_rule() == Rule::not_operator {
                    not_count += 1;
                    next_pair = inner.next().unwrap();
                }

                let mut expression: Expression = match next_pair.as_rule() {
                    Rule::identifier => {
                        Self::Identifier(next_pair.as_str().chars().next().unwrap())
                    }

                    Rule::left_parenthesis => {
                        let inner_expression = inner.next().unwrap();
                        Self::ast(inner_expression)
                    }

                    _ => unreachable!(),
                };

                if not_count % 2 != 0 {
                    expression = Self::Not(Box::new(expression));
                }

                expression
            }

            Rule::and_clause => {
                let mut inner = pair.into_inner();
                let mut left = Self::ast(inner.next().unwrap());

                while let Some(operator) = inner.next() {
                    let right = Self::ast(inner.next().unwrap());
                    left = match operator.as_rule() {
                        Rule::and_operator => Self::And(Box::new(left), Box::new(right)),
                        Rule::nand_operator => Self::Nand(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                }

                left
            }

            Rule::xor_clause => {
                let mut inner = pair.into_inner();
                let mut left = Self::ast(inner.next().unwrap());

                while let Some(operator) = inner.next() {
                    let right = Expression::ast(inner.next().unwrap());
                    left = match operator.as_rule() {
                        Rule::xor_operator => Self::Xor(Box::new(left), Box::new(right)),
                        Rule::xnor_operator => Self::Xnor(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                }

                left
            }

            Rule::expression => {
                let mut inner = pair.into_inner();
                let mut left = Self::ast(inner.next().unwrap());

                while let Some(operator) = inner.next() {
                    let right = Self::ast(inner.next().unwrap());
                    left = match operator.as_rule() {
                        Rule::or_operator => Self::Or(Box::new(left), Box::new(right)),
                        Rule::nor_operator => Self::Nor(Box::new(left), Box::new(right)),
                        _ => unreachable!(),
                    };
                }

                left
            }

            Rule::file => Self::ast(pair.into_inner().next().unwrap()),

            _ => unreachable!(),
        }
    }

    pub fn variables(&self) -> Vec<char> {
        let mut variables = HashSet::new();
        self.all_variables_set(&mut variables);

        let mut variables_vec: Vec<char> = variables.into_iter().collect();
        variables_vec.sort();
        variables_vec
    }

    fn all_variables_set(&self, variables: &mut HashSet<char>) {
        match self {
            Expression::Identifier(ident) => {
                variables.insert(*ident);
            }
            Expression::Not(expr) => {
                expr.all_variables_set(variables);
            }
            Expression::And(left, right)
            | Expression::Nand(left, right)
            | Expression::Or(left, right)
            | Expression::Nor(left, right)
            | Expression::Xor(left, right)
            | Expression::Xnor(left, right) => {
                left.all_variables_set(variables);
                right.all_variables_set(variables);
            }
        }
    }
}

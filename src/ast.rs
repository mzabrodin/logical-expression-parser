use crate::parser::Rule;
use pest::iterators::Pair;
use std::collections::{HashMap, HashSet};

/// An AST node that represents a logical expression recursively.
#[derive(Debug)]
pub enum Expression {
    /// A `char` identifier.
    Identifier(char),
    /// A unary NOT operation.
    Not(Box<Expression>),
    /// A binary AND operation
    And(Box<Expression>, Box<Expression>),
    /// A binary NAND operation
    Nand(Box<Expression>, Box<Expression>),
    /// A binary OR operation
    Or(Box<Expression>, Box<Expression>),
    /// A binary NOR operation
    Nor(Box<Expression>, Box<Expression>),
    /// A binary XOR operation
    Xor(Box<Expression>, Box<Expression>),
    /// A binary XNOR operation
    Xnor(Box<Expression>, Box<Expression>),
}

impl Expression {
    /// Evaluates a logical expression with given variables and return boolean.
    ///
    /// # Arguments
    /// * `variables` - `HashMap<char, bool>` that maps an identifier to its boolean value.
    ///
    /// # Returns
    /// A `bool` result of the expression evaluated with given variables.
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

    /// Recursively creates an [Expression] from result of Pest parsing.
    ///
    /// # Arguments
    /// * `pair` - A `Pair<Rule>` from [pest] crate representing a rule from grammar.
    ///
    /// # Returns
    /// The corresponding [Expression] (AST node).
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

    /// Collects all unique identifiers from the AST.
    ///
    /// # Returns
    /// A sorted `Vec<char>` of all unique variables.
    pub fn variables(&self) -> Vec<char> {
        let mut variables = HashSet::new();
        self.all_variables_set(&mut variables);

        let mut variables_vec: Vec<char> = variables.into_iter().collect();
        variables_vec.sort();
        variables_vec
    }

    /// Helper method for recursively collecting identifiers into a set.
    ///
    /// # Arguments
    /// * `variables` - A mutable `HashSet<char>` to insert all found identifier into.
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

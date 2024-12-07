use std::collections::HashMap;

use super::scalar_type::ScalarType;

/// Expression tree.
#[derive(Debug, Clone)]
pub enum Expr {
    Constant(ScalarType),           // Leaf node: a constant value.
    Variable(String),               // Leaf node: a variable (e.g., "x").
    Add(Box<Expr>, Box<Expr>),      // Binary operation: addition.
    Multiply(Box<Expr>, Box<Expr>), // Binary operation: multiplication.
    Negate(Box<Expr>),              // Unary operation: negation.
    Pow(Box<Expr>, Box<Expr>),      // Binary operation: exponentiation.
}

impl Expr {
    /// Evaluate the expression tree.
    pub fn evaluate(&self, variables: &HashMap<String, ScalarType>) -> ScalarType {
        match self {
            Expr::Constant(value) => *value,
            Expr::Variable(name) => variables[name],
            Expr::Add(left, right) => left.evaluate(variables) + right.evaluate(variables),
            Expr::Multiply(left, right) => left.evaluate(variables) * right.evaluate(variables),
            Expr::Negate(inner) => -inner.evaluate(variables),
            Expr::Pow(base, exponent) => {
                base.evaluate(variables).powf(exponent.evaluate(variables))
            }
        }
    }
}

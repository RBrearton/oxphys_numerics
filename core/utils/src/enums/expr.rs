use std::collections::HashMap;

use super::scalar_type::ScalarType;

/// Expression tree.
#[derive(Debug, Clone)]
pub enum Expr {
    // Leaf nodes.
    Constant(ScalarType), // Leaf node: a constant value.
    Variable(String),     // Leaf node: a variable (e.g., "x").

    // Unary operations.
    Negate(Box<Expr>),         // Unary operation: negation.
    Sqrt(Box<Expr>),           // Unary operation: square root.
    Sin(Box<Expr>),            // Unary operation: sine.
    Cos(Box<Expr>),            // Unary operation: cosine.
    Exp(Box<Expr>),            // Unary operation: exponentiate.
    Log(Box<Expr>, Box<Expr>), // Binary operation: logarithm.
    Ln(Box<Expr>),             // Unary operation: natural logarithm.

    // Binary operations.
    Add(Box<Expr>, Box<Expr>),      // Binary operation: addition.
    Multiply(Box<Expr>, Box<Expr>), // Binary operation: multiplication.
    Pow(Box<Expr>, Box<Expr>),      // Binary operation: exponentiation.
}

impl Expr {
    /// Evaluate the expression tree.
    pub fn evaluate(&self, variables: &HashMap<String, ScalarType>) -> ScalarType {
        match self {
            // Leaf nodes.
            Expr::Constant(value) => *value,
            Expr::Variable(name) => variables[name],

            // Unary operations.
            Expr::Negate(inner) => -inner.evaluate(variables),
            Expr::Sqrt(inner) => inner.evaluate(variables).sqrt(),
            Expr::Sin(inner) => inner.evaluate(variables).sin(),
            Expr::Cos(inner) => inner.evaluate(variables).cos(),
            Expr::Exp(inner) => inner.evaluate(variables).exp(),
            Expr::Log(base, inner) => inner.evaluate(variables).log(base.evaluate(variables)),
            Expr::Ln(inner) => inner.evaluate(variables).ln(),

            // Binary operations.
            Expr::Add(left, right) => left.evaluate(variables) + right.evaluate(variables),
            Expr::Multiply(left, right) => left.evaluate(variables) * right.evaluate(variables),
            Expr::Pow(base, exponent) => {
                base.evaluate(variables).powf(exponent.evaluate(variables))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        // Set up the variables hashmap.
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), ScalarType::F64(2.0));

        // f(x) = x + 1
        let expr = Expr::Add(
            Box::new(Expr::Variable("x".to_string())),
            Box::new(Expr::Constant(ScalarType::F64(1.0))),
        );
        assert_eq!(expr.evaluate(&variables), ScalarType::F64(3.0));

        // f(x) = x^2
        let expr = Expr::Pow(
            Box::new(Expr::Variable("x".to_string())),
            Box::new(Expr::Constant(ScalarType::F64(2.0))),
        );
        assert_eq!(expr.evaluate(&variables), ScalarType::F64(4.0));

        // f(x) = sin(x)
        let expr = Expr::Sin(Box::new(Expr::Variable("x".to_string())));
        assert_eq!(expr.evaluate(&variables), ScalarType::F64(2.0).sin());
    }
}

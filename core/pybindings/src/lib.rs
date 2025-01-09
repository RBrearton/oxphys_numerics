use pyo3::prelude::*;

use oxphys_numerics::{
    enums::{binary_node::BinaryNode, expr::Expr, leaf_node::LeafNode, unary_node::UnaryNode},
    traits::expression::Expression,
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyExpr {
    pub inner: Expr,
}

#[pymethods]
impl PyExpr {
    /// # Evaluate vec
    /// Compile the expression and pass in a vector of values to evaluate the expression.
    pub fn evaluate_vec(&self, variables: Vec<Vec<f64>>) -> Vec<f64> {
        // Jit-compile the expression.
        let f = self.inner.compile_nd().unwrap();

        // Create an output vector of the same length as the input vectors.
        let mut output = vec![0.0; variables[0].len()];

        // Evaluate the expression on each set of input values.
        for (i, values) in variables.iter().enumerate() {
            output[i] = f(values.as_ptr(), values.len());
        }

        output
    }

    // Simple constructors for leaf nodes.
    #[staticmethod]
    pub fn constant(value: f64) -> Self {
        PyExpr {
            inner: Expr::Leaf(LeafNode::Constant(value)),
        }
    }

    #[staticmethod]
    pub fn variable(index: usize) -> Self {
        PyExpr {
            inner: Expr::Leaf(LeafNode::Variable(index)),
        }
    }

    // Constructors for unary nodes.
    #[staticmethod]
    pub fn negate(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Negate(Box::new(child.inner.clone()))),
        }
    }

    #[staticmethod]
    pub fn sqrt(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Sqrt(Box::new(child.inner.clone()))),
        }
    }

    #[staticmethod]
    pub fn sin(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Sin(Box::new(child.inner.clone()))),
        }
    }

    #[staticmethod]
    pub fn cos(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Cos(Box::new(child.inner.clone()))),
        }
    }

    #[staticmethod]
    pub fn exp(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Exp(Box::new(child.inner.clone()))),
        }
    }

    #[staticmethod]
    pub fn ln(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Ln(Box::new(child.inner.clone()))),
        }
    }

    // Constructors for binary nodes.
    #[staticmethod]
    pub fn add(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Add(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            )),
        }
    }

    #[staticmethod]
    pub fn subtract(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Subtract(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            )),
        }
    }

    #[staticmethod]
    pub fn multiply(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Multiply(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            )),
        }
    }

    #[staticmethod]
    pub fn frac(numerator: &PyExpr, denominator: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Frac(
                Box::new(numerator.inner.clone()),
                Box::new(denominator.inner.clone()),
            )),
        }
    }

    #[staticmethod]
    pub fn pow(base: &PyExpr, exponent: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Pow(
                Box::new(base.inner.clone()),
                Box::new(exponent.inner.clone()),
            )),
        }
    }

    #[staticmethod]
    pub fn log(base: &PyExpr, argument: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Log(
                Box::new(base.inner.clone()),
                Box::new(argument.inner.clone()),
            )),
        }
    }

    // Example of how to inspect the stored enum from Python.
    pub fn is_leaf(&self) -> bool {
        matches!(self.inner, Expr::Leaf(_))
    }

    // Optionally get details, e.g. which leaf node type?
    pub fn as_constant(&self) -> Option<f64> {
        match &self.inner {
            Expr::Leaf(LeafNode::Constant(val)) => Some(*val),
            _ => None,
        }
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn oxphys_numerics_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<PyExpr>()?;
    Ok(())
}

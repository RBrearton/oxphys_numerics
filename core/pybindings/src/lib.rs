use pyo3::prelude::*;

use oxphys_numerics::enums::{
    binary_node::BinaryNode, expr::Expr, leaf_node::LeafNode, unary_node::UnaryNode,
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyExpr {
    pub inner: Expr,
}

#[pymethods]
impl PyExpr {
    // Simple constructors for leaf nodes
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

    // Constructors for unary nodes
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

    // Constructors for binary nodes
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
    pub fn sub(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Subtract(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            )),
        }
    }

    // ... etc for mul, div, etc ...

    // Example of how to introspect the stored enum from Python
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

    // You can define similar “getter” methods for variables, or for unaries, or for left/right sides of a binary, etc.
    // ...
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

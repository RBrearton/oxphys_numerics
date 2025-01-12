use numpy::{
    ndarray::{self, s, Dim},
    PyArray, PyArray1, PyArrayMethods, PyReadonlyArray2,
};
use pyo3::{prelude::*, IntoPyObjectExt};

use oxphys_numerics::{
    enums::{binary_node::BinaryNode, expr::Expr, leaf_node::LeafNode, unary_node::UnaryNode},
    traits::expression::Expression,
};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PyExpr {
    pub inner: Expr,
    compiled_fn: Option<fn(*const f64, usize) -> f64>,
}

#[pymethods]
impl PyExpr {
    /// # Evaluate vec
    /// Compile the expression and pass in a NumPy array of values to evaluate the expression.
    pub fn evaluate_vec<'py>(
        &mut self,
        py: Python<'py>,
        variables: PyReadonlyArray2<f64>,
        parallel: bool,
    ) -> PyResult<Bound<'py, PyArray<f64, Dim<[usize; 1]>>>> {
        // Jit-compile the expression, if it hasn't been compiled yet.
        let f = match self.compiled_fn {
            Some(f) => f,
            None => {
                let f = self.inner.compile_nd().unwrap();
                self.compiled_fn = Some(f);
                f
            }
        };

        // Get the number of rows and columns in the input array.
        let dims = variables.dims();
        let (rows, cols) = (dims[0], dims[1]);

        // Create an output array of the same length as the number of rows in the input array.
        // This output array can just be a vector of f64s, and we'll deal with the numpy conversion
        // later.
        let mut output = Vec::with_capacity(rows as usize);
        unsafe {
            output.set_len(rows as usize);
        }

        // Extract raw data before parallel processing. We need to do this because we can't use
        // a PyArray in a multi-threaded context.
        let variables_array = variables.as_array();

        if parallel {
            // If we're running in parallel, we can use Rayon to parallelize the evaluation.
            output.par_iter_mut().enumerate().for_each(|(i, value)| {
                // Create slice without referencing PyArray. Instead, we use the variables_array,
                // which can be safely shared between threads.
                let row_slice = variables_array.slice(s![i, ..]);
                *value = f(row_slice.as_ptr(), cols);
            });
        } else {
            output.iter_mut().enumerate().for_each(|(i, value)| {
                let values = variables_array.slice(s![i, ..]);
                *value = f(values.as_ptr(), cols);
            });
        }

        // Now we need to convert the output vector to a numpy array.
        let output_array: Result<Bound<'_, numpy::PyArray<f64, ndarray::Dim<[usize; 1]>>>, PyErr> =
            PyArray1::from_vec(py, output).into_pyobject_or_pyerr(py);
        output_array
    }

    // Simple constructors for leaf nodes.
    #[staticmethod]
    pub fn constant(value: f64) -> Self {
        PyExpr {
            inner: Expr::Leaf(LeafNode::Constant(value)),
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn variable(index: usize) -> Self {
        PyExpr {
            inner: Expr::Leaf(LeafNode::Variable(index)),
            compiled_fn: None,
        }
    }

    // Constructors for unary nodes.
    #[staticmethod]
    pub fn negate(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Negate(Box::new(child.inner.clone()))),
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn sqrt(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Sqrt(Box::new(child.inner.clone()))),
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn sin(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Sin(Box::new(child.inner.clone()))),
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn cos(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Cos(Box::new(child.inner.clone()))),
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn exp(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Exp(Box::new(child.inner.clone()))),
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn ln(child: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Unary(UnaryNode::Ln(Box::new(child.inner.clone()))),
            compiled_fn: None,
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
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn subtract(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Subtract(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            )),
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn multiply(left: &PyExpr, right: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Multiply(
                Box::new(left.inner.clone()),
                Box::new(right.inner.clone()),
            )),
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn frac(numerator: &PyExpr, denominator: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Frac(
                Box::new(numerator.inner.clone()),
                Box::new(denominator.inner.clone()),
            )),
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn pow(base: &PyExpr, exponent: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Pow(
                Box::new(base.inner.clone()),
                Box::new(exponent.inner.clone()),
            )),
            compiled_fn: None,
        }
    }

    #[staticmethod]
    pub fn log(base: &PyExpr, argument: &PyExpr) -> Self {
        PyExpr {
            inner: Expr::Binary(BinaryNode::Log(
                Box::new(base.inner.clone()),
                Box::new(argument.inner.clone()),
            )),
            compiled_fn: None,
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

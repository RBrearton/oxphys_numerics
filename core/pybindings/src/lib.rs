use pyo3::prelude::*;

#[pyclass]
pub enum ExprType {
    // Leaf nodes.
    Constant,
    Variable,

    // Unary nodes.
    Negate,
    Sqrt,
    Sin,
    Cos,
    Exp,
    Ln,

    // Binary nodes.
    Add,
    Minus,
    Multiply,
    Power,
    Log,
    Fraction,
}

struct Instruction {
    opcode: ExprType, // The thing we want to run.
    arg1: ExprType,   // The first argument; something we may also need to run.
    arg2: ExprType,   // The second argument;
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
    m.add_class::<ExprType>()?;
    Ok(())
}

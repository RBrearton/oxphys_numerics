use std::ops::{Add, Div, Mul, Sub};

/// An enum to represent a scalar value.
#[derive(Clone, Copy, Debug)]
pub enum ScalarType {
    F64(f64),
    F32(f32),
}

/// Create a macro to help implement the arithmetic traits for the ScalarType enum.
macro_rules! impl_arith_trait {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for ScalarType {
            type Output = Self;

            fn $method(self, other: Self) -> Self {
                match (self, other) {
                    (ScalarType::F64(left), ScalarType::F64(right)) => ScalarType::F64(left $op right),
                    (ScalarType::F32(left), ScalarType::F32(right)) => ScalarType::F32(left $op right),
                    _ => panic!("Cannot perform operation on two different scalar types."),
                }
            }
        }
    };
}

// Implement the Add, Sub, Mul, and Div traits for the ScalarType enum.
impl_arith_trait!(Add, add, +);
impl_arith_trait!(Sub, sub, -);
impl_arith_trait!(Mul, mul, *);
impl_arith_trait!(Div, div, /);

/// Implement the From trait for f64 and f32 to allow implicit conversion to ScalarType.
impl From<f64> for ScalarType {
    fn from(value: f64) -> Self {
        ScalarType::F64(value)
    }
}

impl From<f32> for ScalarType {
    fn from(value: f32) -> Self {
        ScalarType::F32(value)
    }
}

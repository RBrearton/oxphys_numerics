use std::ops::{Add, Div, Mul, Neg, Sub};

/// An enum to represent a scalar value.
#[derive(Clone, Copy, Debug, PartialEq)]
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
impl Neg for ScalarType {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            ScalarType::F64(value) => ScalarType::F64(-value),
            ScalarType::F32(value) => ScalarType::F32(-value),
        }
    }
}

/// Add the powf function to the ScalarType enum.
impl ScalarType {
    pub fn powf(self, exponent: ScalarType) -> Self {
        match (self, exponent) {
            (ScalarType::F64(base), ScalarType::F64(exp)) => ScalarType::F64(base.powf(exp)),
            (ScalarType::F32(base), ScalarType::F32(exp)) => ScalarType::F32(base.powf(exp)),
            // If the types are different, convert to the higher precision type.
            (ScalarType::F32(base), ScalarType::F64(exp)) => {
                ScalarType::F64(f64::from(base).powf(exp))
            }
            (ScalarType::F64(base), ScalarType::F32(exp)) => {
                ScalarType::F64(base.powf(f64::from(exp)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // Test addition for f64 and f32.
        assert_eq!(
            ScalarType::F64(1.0) + ScalarType::F64(2.0),
            ScalarType::F64(3.0)
        );
        assert_eq!(
            ScalarType::F32(1.0) + ScalarType::F32(2.0),
            ScalarType::F32(3.0)
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            ScalarType::F64(5.0) - ScalarType::F64(3.0),
            ScalarType::F64(2.0)
        );
        assert_eq!(
            ScalarType::F32(5.0) - ScalarType::F32(3.0),
            ScalarType::F32(2.0)
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            ScalarType::F64(2.0) * ScalarType::F64(3.0),
            ScalarType::F64(6.0)
        );
        assert_eq!(
            ScalarType::F32(2.0) * ScalarType::F32(3.0),
            ScalarType::F32(6.0)
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(
            ScalarType::F64(6.0) / ScalarType::F64(2.0),
            ScalarType::F64(3.0)
        );
        assert_eq!(
            ScalarType::F32(6.0) / ScalarType::F32(2.0),
            ScalarType::F32(3.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_add_different_types() {
        let _ = ScalarType::F64(1.0) + ScalarType::F32(2.0);
    }

    #[test]
    fn test_neg() {
        assert_eq!(-ScalarType::F64(1.0), ScalarType::F64(-1.0));
        assert_eq!(-ScalarType::F32(1.0), ScalarType::F32(-1.0));
    }

    #[test]
    fn test_powf() {
        assert_eq!(
            ScalarType::F64(2.0).powf(ScalarType::F64(3.0)),
            ScalarType::F64(8.0)
        );
        assert_eq!(
            ScalarType::F32(2.0).powf(ScalarType::F32(3.0)),
            ScalarType::F32(8.0)
        );
        assert_eq!(
            ScalarType::F32(2.0).powf(ScalarType::F64(3.0)),
            ScalarType::F64(8.0)
        );
        assert_eq!(
            ScalarType::F64(2.0).powf(ScalarType::F32(3.0)),
            ScalarType::F64(8.0)
        );
    }

    #[test]
    fn test_from() {
        let f64_value: ScalarType = 1.0f64.into();
        let f32_value: ScalarType = 1.0f32.into();
        assert_eq!(f64_value, ScalarType::F64(1.0));
        assert_eq!(f32_value, ScalarType::F32(1.0));
    }
}

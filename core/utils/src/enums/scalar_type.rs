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
                    (ScalarType::F64(left), ScalarType::F64(right)) => {
                        ScalarType::F64(left $op right)
                    }
                    (ScalarType::F32(left), ScalarType::F32(right)) => {
                        ScalarType::F32(left $op right)
                    }

                    // If the types are different, convert to the higher precision type.
                    (ScalarType::F32(left), ScalarType::F64(right)) => {
                        ScalarType::F64(f64::from(left) $op right)
                    }
                    (ScalarType::F64(left), ScalarType::F32(right)) => {
                        ScalarType::F64(left $op f64::from(right))
                    }
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

    pub fn sqrt(self) -> Self {
        match self {
            ScalarType::F64(value) => ScalarType::F64(value.sqrt()),
            ScalarType::F32(value) => ScalarType::F32(value.sqrt()),
        }
    }

    pub fn sin(self) -> Self {
        match self {
            ScalarType::F64(value) => ScalarType::F64(value.sin()),
            ScalarType::F32(value) => ScalarType::F32(value.sin()),
        }
    }

    pub fn cos(self) -> Self {
        match self {
            ScalarType::F64(value) => ScalarType::F64(value.cos()),
            ScalarType::F32(value) => ScalarType::F32(value.cos()),
        }
    }

    pub fn exp(self) -> Self {
        match self {
            ScalarType::F64(value) => ScalarType::F64(value.exp()),
            ScalarType::F32(value) => ScalarType::F32(value.exp()),
        }
    }

    pub fn log(self, base: Self) -> Self {
        match (self, base) {
            (ScalarType::F64(inner), ScalarType::F64(base)) => ScalarType::F64(inner.log(base)),
            (ScalarType::F32(inner), ScalarType::F32(base)) => ScalarType::F32(inner.log(base)),
            // If the types are different, convert to the higher precision type.
            (ScalarType::F32(inner), ScalarType::F64(base)) => {
                ScalarType::F64(f64::from(inner).log(base))
            }
            (ScalarType::F64(inner), ScalarType::F32(base)) => {
                ScalarType::F64(inner.log(f64::from(base)))
            }
        }
    }

    pub fn ln(self) -> Self {
        match self {
            ScalarType::F64(value) => ScalarType::F64(value.ln()),
            ScalarType::F32(value) => ScalarType::F32(value.ln()),
        }
    }

    pub fn abs(self) -> Self {
        match self {
            ScalarType::F64(value) => ScalarType::F64(value.abs()),
            ScalarType::F32(value) => ScalarType::F32(value.abs()),
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
        assert_eq!(
            ScalarType::F32(1.0) + ScalarType::F64(2.0),
            ScalarType::F64(3.0)
        );
        assert_eq!(
            ScalarType::F64(1.0) + ScalarType::F32(2.0),
            ScalarType::F64(3.0)
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
        assert_eq!(
            ScalarType::F32(5.0) - ScalarType::F64(3.0),
            ScalarType::F64(2.0)
        );
        assert_eq!(
            ScalarType::F64(5.0) - ScalarType::F32(3.0),
            ScalarType::F64(2.0)
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
        assert_eq!(
            ScalarType::F32(2.0) * ScalarType::F64(3.0),
            ScalarType::F64(6.0)
        );
        assert_eq!(
            ScalarType::F64(2.0) * ScalarType::F32(3.0),
            ScalarType::F64(6.0)
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
        assert_eq!(
            ScalarType::F32(6.0) / ScalarType::F64(2.0),
            ScalarType::F64(3.0)
        );
        assert_eq!(
            ScalarType::F64(6.0) / ScalarType::F32(2.0),
            ScalarType::F64(3.0)
        );
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
    fn test_sqrt() {
        assert_eq!(ScalarType::F64(4.0).sqrt(), ScalarType::F64(2.0));
        assert_eq!(ScalarType::F32(4.0).sqrt(), ScalarType::F32(2.0));
    }

    #[test]
    fn test_sin() {
        // These tests are pretty weak.
        assert_eq!(ScalarType::F64(0.0).sin(), ScalarType::F64(0.0));
        assert_eq!(ScalarType::F32(0.0).sin(), ScalarType::F32(0.0));
    }

    #[test]
    fn test_cos() {
        // These tests are pretty weak.
        assert_eq!(ScalarType::F64(0.0).cos(), ScalarType::F64(1.0));
        assert_eq!(ScalarType::F32(0.0).cos(), ScalarType::F32(1.0));
    }

    #[test]
    fn test_exp() {
        // These tests are pretty weak.
        assert_eq!(ScalarType::F64(0.0).exp(), ScalarType::F64(1.0));
        assert_eq!(ScalarType::F32(0.0).exp(), ScalarType::F32(1.0));
    }

    #[test]
    fn test_log() {
        assert_eq!(
            ScalarType::F64(8.0).log(ScalarType::F64(2.0)),
            ScalarType::F64(3.0)
        );
        assert_eq!(
            ScalarType::F32(8.0).log(ScalarType::F32(2.0)),
            ScalarType::F32(3.0)
        );
        assert_eq!(
            ScalarType::F32(8.0).log(ScalarType::F64(2.0)),
            ScalarType::F64(3.0)
        );
        assert_eq!(
            ScalarType::F64(8.0).log(ScalarType::F32(2.0)),
            ScalarType::F64(3.0)
        );
    }

    #[test]
    fn test_ln() {
        // These tests are pretty weak.
        assert_eq!(ScalarType::F64(1.0).ln(), ScalarType::F64(0.0));
        assert_eq!(ScalarType::F32(1.0).ln(), ScalarType::F32(0.0));
    }

    #[test]
    fn test_from() {
        let f64_value: ScalarType = 1.0f64.into();
        let f32_value: ScalarType = 1.0f32.into();
        assert_eq!(f64_value, ScalarType::F64(1.0));
        assert_eq!(f32_value, ScalarType::F32(1.0));
    }
}

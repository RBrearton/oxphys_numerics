use super::{initialized_expr::InitializedExpr, uninitialized_expr::UninitializedExpr};

/// # Expr
/// The main expression enum. This represents a node in an `oxphys_numerics` expression tree.
#[derive(Debug, Clone)]
pub enum Expr {
    Initialized(InitializedExpr),
    Uninitialized(UninitializedExpr),
}

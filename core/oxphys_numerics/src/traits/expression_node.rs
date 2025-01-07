use crate::enums::expr::Expr;

pub trait ExpressionNode {
    /// # To expr
    /// Convert this expression node to an `Expr`.
    fn to_expr(&self) -> Expr;
}

use cranelift_codegen::ir::Value;
use cranelift_frontend::FunctionBuilder;

/// # Expression compiler
/// This trait defines all the methods that need to be implemented to make an it possible to
/// jit-compile an expression.
pub(crate) trait ExpressionCompiler {
    /// # Build jit 1D
    /// Given a jit function builder, add this expression to the function builder. This is called
    /// when we build an expression that takes a single f64 input.
    fn build_jit_1d(&self, builder: &mut FunctionBuilder, parameter: Value) -> Value;

    /// # Build jit 2D
    /// Given a jit function builder, add this expression to the function builder. This is called
    /// when we build an expression that takes two f64 inputs.
    fn build_jit_2d(&self, builder: &mut FunctionBuilder, param_0: Value, param_1: Value) -> Value;

    /// # Build jit 3D
    /// Given a jit function builder, add this expression to the function builder. This is called
    /// when we build an expression that takes three f64 inputs.
    fn build_jit_3d(
        &self,
        builder: &mut FunctionBuilder,
        param_0: Value,
        param_1: Value,
        param_2: Value,
    ) -> Value;

    /// # Build jit ND
    /// Given a jit function builder, add this expression to the function builder. This is called
    /// when we build an expression that takes an ND input (i.e. an array of f64s).
    fn build_jit_nd(&self, builder: &mut FunctionBuilder, parameters: &[Value]) -> Value;
}

use crate::errors::expr_parsing_error::ExprParsingError;
use cranelift_codegen::ir::{types, Value};
use cranelift_codegen::ir::{AbiParam, InstBuilder, Signature};
use cranelift_codegen::isa::CallConv;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use cranelift_native;

/// Type alias for a compiled expression function that maps `f64` to `f64`.
pub type CompiledExpression = fn(*const f64, usize) -> f64;

/// # Expression
/// This defines everything that we expect from our data structures that represent mathematical
/// expressions.
pub trait Expression {
    /// # Evaluate
    /// Run the expression with the given variables and return the result.
    ///
    /// ## Warning - slow
    /// If performance is critical, please consider compiling the expression with the `compile`
    /// function and calling the compiled function directly.
    fn evaluate(&self, variables: &Vec<f64>) -> f64;

    /// # Build jit
    /// Given a jit function builder, add this expression to the function builder.
    fn build_jit(&self, builder: &mut FunctionBuilder, parameters: &[Value]) -> Value;

    /// # Number of variables
    /// Get the number of independent variables in the expression. This can be easily figured out
    /// by the maximum index of the variables in the expression.
    fn num_variables(&self) -> usize;

    /// # Compile
    /// Just-in-time compile the expression. This is slower than evaluate if you're only going to
    /// call the expression once, but it is *much* faster if you're going to call the expression
    /// many times.
    fn compile(&self) -> Result<CompiledExpression, ExprParsingError> {
        // Use cranelift_native to configure ISA for your current platform (e.g. Apple Silicon).
        let isa_builder = cranelift_native::builder().expect("Failed to create ISA builder");

        // Create a default flags builder and then Flags.
        let flag_builder = cranelift_codegen::settings::builder();
        let flags = cranelift_codegen::settings::Flags::new(flag_builder);
        let isa = isa_builder.finish(flags).expect("Failed to create ISA");

        // Create a JIT builder with the appropriate ISA.
        let jit_builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let mut module = JITModule::new(jit_builder);

        // Create the function signature f([x1, x2, ...], len(array)) -> f64.
        let mut function_signature =
            Signature::new(CallConv::triple_default(module.isa().triple()));

        // The pointer type depends on the ISA; module.isa().pointer_type() gives the correct type.
        let ptr_type = module.isa().pointer_type();

        // Add a pointer parameter and a length parameter.
        function_signature.params.push(AbiParam::new(ptr_type));
        function_signature.params.push(AbiParam::new(types::I64));

        // Add the return value.
        function_signature.returns.push(AbiParam::new(types::F64));

        // Declare the function.
        let function_id = module
            .declare_function("f", Linkage::Local, &function_signature)
            .unwrap();

        // Prepare the function context.
        let mut context = module.make_context();
        context.func.signature = function_signature;
        let mut function_context = FunctionBuilderContext::new();

        // Build the function IR.
        {
            // Make the function builder.
            let mut builder = FunctionBuilder::new(&mut context.func, &mut function_context);

            // Create the entry block. This is where the function starts, and it has the parameters
            // that we need.
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // Get the parameters.
            let params_slice = builder.block_params(entry_block);

            // Copy them into a standalone vector. This separates the lifetimes of the parameters
            // from the lifetime of the builder, needed because we used an immutable borrow of the
            // builder to make the parameters.
            let parameters = params_slice.to_vec();

            // Pass the parameters and the builder to the expression to build itself recursively.
            let return_value = self.build_jit(&mut builder, &parameters);
            builder.ins().return_(&[return_value]);
            builder.finalize();
        }

        // Define and finalize the function.
        module.define_function(function_id, &mut context).unwrap();
        module.clear_context(&mut context);
        module.finalize_definitions().unwrap();

        // Get a callable function pointer.
        let code = module.get_finalized_function(function_id);
        let compiled_function =
            unsafe { std::mem::transmute::<_, fn(*const f64, usize) -> f64>(code) };
        Ok(compiled_function)
    }
}

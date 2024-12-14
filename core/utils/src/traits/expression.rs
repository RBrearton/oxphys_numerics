use crate::errors::expr_parsing_error::ExprParsingError;
use cranelift_codegen::ir::{types, Value};
use cranelift_codegen::ir::{AbiParam, InstBuilder, Signature};
use cranelift_codegen::isa::CallConv;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use cranelift_native;

/// Type alias for a compiled expression function that maps `f64` to `f64`.
pub type CompiledExpression = fn(f64) -> f64;

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
    fn build_jit(&self, builder: &mut FunctionBuilder) -> Value;

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

        // Create the function signature f(x: f64) -> f64
        let mut sig = Signature::new(CallConv::triple_default(module.isa().triple()));
        sig.params.push(AbiParam::new(types::F64));
        sig.returns.push(AbiParam::new(types::F64));

        let func_id = module.declare_function("f", Linkage::Local, &sig).unwrap();

        // Prepare the function context.
        let mut ctx = module.make_context();
        ctx.func.signature = sig;
        let mut func_ctx = FunctionBuilderContext::new();

        // Build the function IR.
        {
            let mut builder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            let x = builder.block_params(entry_block)[0];
            let one = builder.ins().f64const(1.0);
            let added = builder.ins().fadd(x, one);

            builder.ins().return_(&[added]);
            builder.finalize();
        }

        // Define and finalize the function.
        module.define_function(func_id, &mut ctx).unwrap();
        module.clear_context(&mut ctx);
        module.finalize_definitions().unwrap();

        // Get a callable function pointer.
        let code = module.get_finalized_function(func_id);
        let compiled_function = unsafe { std::mem::transmute::<_, fn(f64) -> f64>(code) };
        Ok(compiled_function)
    }
}

use crate::errors::expr_parsing_error::ExprParsingError;
use crate::structs::instruction_set_architecture::InstructionSetArchitecture;
use crate::structs::jit_helper::JITHelper;
use cranelift_codegen::ir::types;
use cranelift_codegen::ir::InstBuilder;

use super::expression_compiler::ExpressionCompiler;

/// Type alias for a compiled expression function that maps a single `f64` to a single `f64`.
pub type CompiledExpression1D = fn(f64) -> f64;

/// Type alias for a compiled expression function that maps two `f64`s to a single `f64`.
pub type CompiledExpression2D = fn(f64, f64) -> f64;

/// Type alias for a compiled expression function that maps three `f64`s to a single `f64`.
pub type CompiledExpression3D = fn(f64, f64, f64) -> f64;

/// Type alias for a compiled expression function that maps an array of `f64`s to a single `f64`.
pub type CompiledExpressionND = fn(*const f64, usize) -> f64;

/// # Expression
/// This defines everything that we expect from our data structures that represent mathematical
/// expressions.
pub trait Expression: ExpressionCompiler {
    /// # Number of variables
    /// Get the number of independent variables in the expression. This can be easily figured out
    /// by the maximum index of the variables in the expression.
    fn num_variables(&self) -> usize;

    /// # Compile 1D
    /// Just-in-time compile the expression. This is slower than evaluate if you're only going to
    /// call the expression once, but it is *much* faster if you're going to call the expression
    /// many times.
    fn compile_1d(&self) -> Result<CompiledExpression1D, ExprParsingError> {
        // Prepare our input arguments, then make a JITHelper.
        let parameters = vec![types::F64];
        let return_type = types::F64;
        let mut jit_helper = JITHelper::new(
            InstructionSetArchitecture::current_platform(),
            parameters,
            return_type,
        );

        // Build the function IR.
        {
            // Make the function builder.
            let mut builder = jit_helper.function_builder();

            // Create the entry block. This is where the function starts, and it has the parameters
            // that we need.
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // Get the parameter.
            let parameter = builder.block_params(entry_block)[0];

            // Pass the parameter and the builder to the expression to build itself recursively.
            let return_value = self.build_jit_1d(&mut builder, parameter);
            builder.ins().return_(&[return_value]);
            builder.finalize();
        }

        // Get a callable function pointer.
        let code = jit_helper.finalize();
        let compiled_function = unsafe { std::mem::transmute::<_, fn(f64) -> f64>(code) };
        Ok(compiled_function)
    }

    /// # Compile 2D
    /// Just-in-time compile the expression. This is slower than evaluate if you're only going to
    /// call the expression once, but it is *much* faster if you're going to call the expression
    /// many times.
    fn compile_2d(&self) -> Result<CompiledExpression2D, ExprParsingError> {
        // Prepare our input arguments, then make a JITHelper.
        let parameters = vec![types::F64, types::F64];
        let return_type = types::F64;
        let mut jit_helper = JITHelper::new(
            InstructionSetArchitecture::current_platform(),
            parameters,
            return_type,
        );

        // Build the function IR.
        {
            // Make the function builder.
            let mut builder = jit_helper.function_builder();

            // Create the entry block. This is where the function starts, and it has the parameters
            // that we need.
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // Get the function parameters.
            let x = builder.block_params(entry_block)[0];
            let y = builder.block_params(entry_block)[1];

            // Pass the parameters and the builder to the expression to build itself recursively.
            let return_value = self.build_jit_2d(&mut builder, x, y);
            builder.ins().return_(&[return_value]);
            builder.finalize();
        }

        // Get a callable function pointer.
        let code = jit_helper.finalize();
        let compiled_function = unsafe { std::mem::transmute::<_, fn(f64, f64) -> f64>(code) };
        Ok(compiled_function)
    }

    /// # Compile 3D
    /// Just-in-time compile the expression. This is slower than evaluate if you're only going to
    /// call the expression once, but it is *much* faster if you're going to call the expression
    /// many times.
    fn compile_3d(&self) -> Result<CompiledExpression3D, ExprParsingError> {
        // Prepare our input arguments, then make a JITHelper.
        let parameters = vec![types::F64, types::F64, types::F64];
        let return_type = types::F64;
        let mut jit_helper = JITHelper::new(
            InstructionSetArchitecture::current_platform(),
            parameters,
            return_type,
        );

        // Build the function IR.
        {
            // Make the function builder.
            let mut builder = jit_helper.function_builder();

            // Create the entry block. This is where the function starts, and it has the parameters
            // that we need.
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // Get the function parameters.
            let x = builder.block_params(entry_block)[0];
            let y = builder.block_params(entry_block)[1];
            let z = builder.block_params(entry_block)[2];

            // Pass the parameters and the builder to the expression to build itself recursively.
            let return_value = self.build_jit_3d(&mut builder, x, y, z);
            builder.ins().return_(&[return_value]);
            builder.finalize();
        }

        // Get a callable function pointer.
        let code = jit_helper.finalize();
        let compiled_function = unsafe { std::mem::transmute::<_, fn(f64, f64, f64) -> f64>(code) };
        Ok(compiled_function)
    }

    /// # Compile ND
    /// Just-in-time compile the expression. This is slower than evaluate if you're only going to
    /// call the expression once, but it is *much* faster if you're going to call the expression
    /// many times.
    fn compile_nd(&self) -> Result<CompiledExpressionND, ExprParsingError> {
        // One of our function's parameters is a pointer. Because these are ISA dependent, start by
        // making an InstructionSetArchitecture instance for our platform.
        let isa = InstructionSetArchitecture::current_platform();

        // Prepare our input arguments, then make a JITHelper.
        let parameters = vec![isa.pointer_type(), types::I64];
        let return_type = types::F64;
        let mut jit_helper = JITHelper::new(isa, parameters, return_type);

        // Build the function IR.
        {
            // Make the function builder.
            let mut builder = jit_helper.function_builder();

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
            let return_value = self.build_jit_nd(&mut builder, &parameters);
            builder.ins().return_(&[return_value]);
            builder.finalize();
        }

        // Get a callable function pointer.
        let code = jit_helper.finalize();
        let compiled_function =
            unsafe { std::mem::transmute::<_, fn(*const f64, usize) -> f64>(code) };
        Ok(compiled_function)
    }
}

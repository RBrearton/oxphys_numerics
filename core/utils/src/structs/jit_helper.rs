use cranelift_codegen::ir::types;
use cranelift_codegen::ir::{AbiParam, Signature};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::Context;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{FuncId, Linkage, Module};

use super::instruction_set_architecture::InstructionSetArchitecture;

/// # Jit helper
/// This struct contains some common cranelift objects that are needed to run the oxphys_numerics
/// jit compilation routines.
pub(crate) struct JITHelper {
    /// The module that we're going to be adding functions to.
    module: JITModule,

    /// The function ID.
    function_id: FuncId,

    /// The function context.
    function_context: FunctionBuilderContext,

    /// The compilation context.
    context: Context,
}

impl JITHelper {
    /// # New
    /// Create a new JITHelper.
    pub(crate) fn new(
        isa: InstructionSetArchitecture,
        parameters: Vec<types::Type>,
        return_type: types::Type,
    ) -> Self {
        // Create a JIT builder with the appropriate ISA.
        let jit_builder = JITBuilder::with_isa(
            isa.cranelift_isa(),
            cranelift_module::default_libcall_names(),
        );

        // Create a new module.
        let mut module = JITModule::new(jit_builder);

        // Create the function signature object, which will be used to declare the function.
        let mut function_signature =
            Signature::new(CallConv::triple_default(module.isa().triple()));

        // Add the parameters and return value to the function signature.
        for parameter in parameters {
            function_signature.params.push(AbiParam::new(parameter));
        }
        function_signature.returns.push(AbiParam::new(return_type));

        // Declare the function.
        let function_id = module
            .declare_function("f", Linkage::Local, &function_signature)
            .unwrap();

        // Prepare the function context.
        let mut context = module.make_context();
        context.func.signature = function_signature.clone();
        let function_context = FunctionBuilderContext::new();

        // Create a new JITHelper.
        JITHelper {
            module,
            function_id,
            function_context,
            context,
        }
    }

    /// # Get function builder
    /// Get a FunctionBuilder object that can be used to build the function's IR.
    pub(crate) fn function_builder(&mut self) -> FunctionBuilder {
        FunctionBuilder::new(&mut self.context.func, &mut self.function_context)
    }

    /// # Finalize
    /// Define the finalized function, and finalize the module.
    pub(crate) fn finalize(&mut self) -> *const u8 {
        // Define and finalize the function.
        self.module
            .define_function(self.function_id, &mut self.context)
            .unwrap();
        self.module.clear_context(&mut self.context);
        self.module.finalize_definitions().unwrap();

        // Return a callable function pointer.
        self.module.get_finalized_function(self.function_id)
    }
}

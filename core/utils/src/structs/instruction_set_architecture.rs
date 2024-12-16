use std::sync::Arc;

use cranelift_codegen::ir::types;
use cranelift_codegen::isa::TargetIsa;
use cranelift_codegen::settings::Configurable;
use cranelift_native;

/// # Instruction set architecture
/// This struct contains information relating to the instruction set architecture that we're
/// compiling for.
pub(crate) struct InstructionSetArchitecture {
    isa: Arc<dyn TargetIsa>,
}

impl InstructionSetArchitecture {
    /// # New
    /// Create a new ISA corresponding to the current platform.
    pub fn current_platform() -> Self {
        // Use cranelift_native to configure ISA for your current platform (e.g. Apple Silicon).
        let isa_builder = cranelift_native::builder().expect("Failed to create ISA builder");

        // Create a default flags builder and manually pass in the opt_level "speed" flag. We're
        // generally very performance sensitive here, so we'll always want the most aggressive
        // optimization level here.
        let mut flag_builder = cranelift_codegen::settings::builder();
        flag_builder.set("opt_level", "speed").unwrap();
        let flags = cranelift_codegen::settings::Flags::new(flag_builder);
        let isa = isa_builder.finish(flags).expect("Failed to create ISA");

        InstructionSetArchitecture { isa }
    }

    /// # Pointer type
    /// The pointer type depends on the ISA, so we return the correct pointer type for the ISA here.
    pub fn pointer_type(&self) -> types::Type {
        self.isa.pointer_type()
    }

    /// # Get cranelift ISA
    /// Return the cranelift ISA.
    pub fn cranelift_isa(&self) -> Arc<dyn TargetIsa> {
        self.isa.clone()
    }
}

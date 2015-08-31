#[test]
fn it_works() {
}

trait Builder {
    /// Build an instruction that converts a float to an unsigned integer of a given type.
    pub fn build_fp_to_ui(&self, value: &Value, dest: &Type) -> &Value;
    /// Build an instruction that converts a float to a signed integer of a given type.
    pub fn build_fp_to_si(&self, value: &Value, dest: &Type) -> &Value;
    /// Build an instruction that converts an unsigned integer to a float of a given type.
    pub fn build_ui_to_fp(&self, value: &Value, dest: &Type) -> &Value;
    /// Build an instruction that converts a signed integer to a float of a given type.
    pub fn build_si_to_fp(&self, value: &Value, dest: &Type) -> &Value;
}

impl Builder for llvm::Builder {
    pub fn build_fp_to_ui(&self, value: &Value, dest: &Type) -> &Value {
        unsafe { core::LLVMBuildFPToUI(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr()).into() }
    }
    pub fn build_fp_to_si(&self, value: &Value, dest: &Type) -> &Value {
        unsafe { core::LLVMBuildFPToSI(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr()).into() }
    }
    pub fn build_ui_to_fp(&self, value: &Value, dest: &Type) -> &Value {
        unsafe { core::LLVMBuildUIToFP(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr()).into() }
    }
    pub fn build_si_to_fp(&self, value: &Value, dest: &Type) -> &Value {
        unsafe { core::LLVMBuildSIToFP(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr()).into() }
    }
}

use llvm_sys::analysis::{LLVMVerifierFailureAction};
use llvm_sys::prelude::{LLVMValueRef, LLVMBool};

#[link(name = "wrapper")]
extern "C" {
    /// Verify that a function is valid, taking the specified action if not.
    ///
    /// Unlike `LLVMVerifyFunction`, this function supports an optional
    /// `OutMessage` argument for storing a description of any invalid
    /// constructs, analogous to that used in `LLVMVerifyModule`.
    ///
    /// Any returned `OutMessage` must then be disposed with `LLVMDisposeMessage`.
    pub fn LLVMVerifyFunctionWithOutput(Fn: LLVMValueRef,
                                    Action: LLVMVerifierFailureAction,
                                    OutMessage: *mut *mut ::libc::c_char) -> LLVMBool;

}

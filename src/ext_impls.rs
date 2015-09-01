use libc::{c_char};

use super::BasicBlock as BasicBlockExt;
use super::Builder as BuilderExt;
use super::Function as FunctionExt;
use llvm::{BasicBlock, Builder, Function, Type, Value};
use llvm_sys::core;
use llvm_sys::analysis::{LLVMVerifierFailureAction};
use llvm_sys::prelude::{LLVMValueRef, LLVMBool};

use std::ffi::CStr;
use std::mem;

static NULL_NAME:[c_char; 1] = [0];

impl BuilderExt for Builder {
    fn build_fp_to_ui(&self, value: &Value, dest: &Type) -> &Value {
        unsafe { core::LLVMBuildFPToUI(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr()).into() }
    }
    fn build_fp_to_si(&self, value: &Value, dest: &Type) -> &Value {
        unsafe { core::LLVMBuildFPToSI(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr()).into() }
    }
    fn build_ui_to_fp(&self, value: &Value, dest: &Type) -> &Value {
        unsafe { core::LLVMBuildUIToFP(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr()).into() }
    }
    fn build_si_to_fp(&self, value: &Value, dest: &Type) -> &Value {
        unsafe { core::LLVMBuildSIToFP(self.into(), value.into(), dest.into(), NULL_NAME.as_ptr()).into() }
    }
}

impl BasicBlockExt for BasicBlock {
    fn get_next(&self) -> Option<&BasicBlock> {
        unsafe { mem::transmute(core::LLVMGetNextBasicBlock(self.into())) }
    }
    fn get_previous(&self) -> Option<&BasicBlock> {
        unsafe { mem::transmute(core::LLVMGetPreviousBasicBlock(self.into())) }
    }

    fn get_terminator(&self) -> Option<&Value> {
        unsafe { ptr_to_null(core::LLVMGetBasicBlockTerminator(self.into())) }
    }

    fn get_first_instruction(&self) -> Option<&Value> {
        unsafe { ptr_to_null(core::LLVMGetFirstInstruction(self.into())) }
    }

    fn get_last_instruction(&self) -> Option<&Value> {
        unsafe { ptr_to_null(core::LLVMGetLastInstruction(self.into())) }
    }
}

impl FunctionExt for Function {
    fn empty(&self) -> bool {
        match self.get_basic_block_count() {
            0 => true,
            1 => self.get_entry().unwrap().get_first_instruction().is_none(),
            _ => false,
        }
    }

    fn get_basic_block_count(&self) -> u32 {
        unsafe { core::LLVMCountBasicBlocks(self.into()) }
    }

    /// Returns the first basic block in this function, or `None` if there is none.
    fn get_first(&self) -> Option<&BasicBlock> {
        unsafe { mem::transmute(core::LLVMGetFirstBasicBlock(self.into())) }
    }
    /// Returns the last basic block in this function, or `None` if there is none.
    fn get_last(&self) -> Option<&BasicBlock> {
        unsafe { mem::transmute(core::LLVMGetLastBasicBlock(self.into())) }
    }

    fn verify(&self) -> Result<(), String> {
        unsafe {
            // let action = LLVMVerifierFailureAction::LLVMReturnStatusAction;
            let mut error = mem::uninitialized();
            let action = LLVMVerifierFailureAction::LLVMPrintMessageAction;
            if 1 == LLVMVerifyFunctionWithOutput(self.into(),
                                                 action,
                                                 &mut error) {
                let c_str = CStr::from_ptr(error);
                let s = c_str.to_string_lossy().to_string();
                core::LLVMDisposeMessage(error);
                Err(s)
            } else {
                Ok(())
            }
        }
    }
}

pub unsafe fn ptr_to_null<P, T>(ptr: *mut P) -> Option<T> where T:From<*mut P> {
    if ptr.is_null() {
        None
    } else {
        Some(ptr.into())
    }
}

#[link(name = "wrapper")]
extern "C" {
    /// Verify that a function is valid, taking the specified action if not.
    ///
    /// Unlike `LLVMVerifyFunction`, this function supports an optional
    /// `OutMessage` argument for storing a description of any invalid
    /// constructs, analogous to that used in `LLVMVerifyModule`.
    ///
    /// Any returned `OutMessage` must then be disposed with `LLVMDisposeMessage`.
    fn LLVMVerifyFunctionWithOutput(Fn: LLVMValueRef,
                                    Action: LLVMVerifierFailureAction,
                                    OutMessage: *mut *mut ::libc::c_char) -> LLVMBool;

}

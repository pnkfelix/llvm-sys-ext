#![feature(cstr_to_str)]

extern crate cbox;
extern crate libc;
extern crate llvm;
extern crate llvm_sys;

pub use self::ext_traits::{Builder, BasicBlock, Function};
pub use self::types::{FunctionPassManager};

mod ext_traits;
mod ext_impls;

mod types;

pub mod sys {
    pub use ::ext_impls::LLVMVerifyFunctionWithOutput;
}

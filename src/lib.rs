#![feature(cstr_to_str)]

extern crate cbox;
extern crate libc;
extern crate llvm;
extern crate llvm_sys;

pub trait Builder {
    /// Build an instruction that converts a float to an unsigned integer of a given type.
    fn build_fp_to_ui(&self, value: &llvm::Value, dest: &llvm::Type) -> &llvm::Value;
    /// Build an instruction that converts a float to a signed integer of a given type.
    fn build_fp_to_si(&self, value: &llvm::Value, dest: &llvm::Type) -> &llvm::Value;
    /// Build an instruction that converts an unsigned integer to a float of a given type.
    fn build_ui_to_fp(&self, value: &llvm::Value, dest: &llvm::Type) -> &llvm::Value;
    /// Build an instruction that converts a signed integer to a float of a given type.
    fn build_si_to_fp(&self, value: &llvm::Value, dest: &llvm::Type) -> &llvm::Value;
}

pub trait BasicBlock {
    /// Returns the next basic block after `self`, or `None` if there is none.
    fn get_next(&self) -> Option<&llvm::BasicBlock>;
    /// Returns the previous basic block before `self`, or `None` if there is none.
    fn get_previous(&self) -> Option<&llvm::BasicBlock>;

    /// Return the terminator instruction if well-formed, or `None` otherwise.
    fn get_terminator(&self) -> Option<&llvm::Value>;

    /// Return the first instruction in the block.
    fn get_first_instruction(&self) -> Option<&llvm::Value>;

    /// Return the last instruction in the block.
    fn get_last_instruction(&self) -> Option<&llvm::Value>;
}

pub trait Function {
    /// Returns true iff this function has no instructions added to it.
    fn empty(&self) -> bool;

    /// Returns the numbr of blocks in this function.
    fn get_basic_block_count(&self) -> u32;

    /// Returns the first basic block in this function, or `None` if there is none.
    fn get_first(&self) -> Option<&llvm::BasicBlock>;

    /// Returns the last basic block in this function, or `None` if there is none.
    fn get_last(&self) -> Option<&llvm::BasicBlock>;

    /// Runs the built-in LLVM consistency checker on this function, returning
    /// Ok if its okay and Err if a problem was encountered.
    fn verify(&self) -> Result<(), String>;
}

mod ext_impls;

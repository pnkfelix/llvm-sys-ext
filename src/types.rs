use cbox::{DisposeRef, CBox};
use llvm::{Function, Module};
use llvm_sys::{LLVMPassManager};
use llvm_sys::core::{LLVMCreateFunctionPassManagerForModule};
use llvm_sys::core::{LLVMInitializeFunctionPassManager};
use llvm_sys::core::{LLVMRunFunctionPassManager};
use llvm_sys::core::{LLVMDisposePassManager};
use llvm_sys::prelude::{LLVMPassManagerRef};
use std::marker::PhantomData;
use std::mem;

pub struct FunctionPassManager<'a> {
    _m: PhantomData<&'a Module>,
}

impl<'a> FunctionPassManager<'a> {
    pub fn into(&self) -> LLVMPassManagerRef {
        unsafe { mem::transmute(self) }
    }
}

impl<'a> DisposeRef for FunctionPassManager<'a> {
    type RefTo = LLVMPassManager;
    unsafe fn dispose(ptr: LLVMPassManagerRef) { LLVMDisposePassManager(ptr) }
}

impl<'a> FunctionPassManager<'a> {
    pub fn new(m: &'a Module) -> CBox<FunctionPassManager<'a>> {
        unsafe {
            CBox::new(LLVMCreateFunctionPassManagerForModule(m.into()))
        }
    }

    pub fn do_initialization(&self) -> bool {
        unsafe {
            LLVMInitializeFunctionPassManager(self.into()) != 0
        }
    }

    pub fn run(&self, f: &Function) -> bool {
        unsafe {
            LLVMRunFunctionPassManager(self.into(), f.into()) != 0
        }
    }
}

macro_rules! forward_scalar_transform {
    ($rname:ident, $cname:ident) => {
        use llvm_sys::transforms::scalar::$cname;
        impl<'a> FunctionPassManager<'a> {
            pub fn $rname(&self) {
                unsafe { $cname(self.into()); }
            }
        }
    }
}

forward_scalar_transform!(
    add_aggressive_dce_pass, LLVMAddAggressiveDCEPass);
forward_scalar_transform!(
    add_alignment_from_assumptions_pass, LLVMAddAlignmentFromAssumptionsPass);
forward_scalar_transform!(
    add_cfg_simplification_pass, LLVMAddCFGSimplificationPass);
forward_scalar_transform!(
    add_dead_store_elimination_pass, LLVMAddDeadStoreEliminationPass);
forward_scalar_transform!(
    add_scalarizer_pass, LLVMAddScalarizerPass);
forward_scalar_transform!(
    add_merged_load_store_motion_pass, LLVMAddMergedLoadStoreMotionPass);
forward_scalar_transform!(
    add_gvn_pass, LLVMAddGVNPass);
forward_scalar_transform!(
    add_ind_var_simplify_pass, LLVMAddIndVarSimplifyPass);
forward_scalar_transform!(
    add_instruction_combining_pass, LLVMAddInstructionCombiningPass);
forward_scalar_transform!(
    add_jump_threading_pass, LLVMAddJumpThreadingPass);
forward_scalar_transform!(
    add_licm_pass, LLVMAddLICMPass);
forward_scalar_transform!(
    add_loop_deletion_pass, LLVMAddLoopDeletionPass);
forward_scalar_transform!(
    add_loop_idiom_pass, LLVMAddLoopIdiomPass);
forward_scalar_transform!(
    add_loop_rotate_pass, LLVMAddLoopRotatePass);
forward_scalar_transform!(
    add_loop_reroll_pass, LLVMAddLoopRerollPass);
forward_scalar_transform!(
    add_loop_unroll_pass, LLVMAddLoopUnrollPass);
forward_scalar_transform!(
    add_loop_unswitch_pass, LLVMAddLoopUnswitchPass);
forward_scalar_transform!(
    add_mem_cpy_opt_pass, LLVMAddMemCpyOptPass);
forward_scalar_transform!(
    add_partially_inline_lib_calls_pass, LLVMAddPartiallyInlineLibCallsPass);
forward_scalar_transform!(
    add_lower_switch_pass, LLVMAddLowerSwitchPass);
forward_scalar_transform!(
    add_promote_memory_to_register_pass, LLVMAddPromoteMemoryToRegisterPass);
forward_scalar_transform!(
    add_reassociate_pass, LLVMAddReassociatePass);
forward_scalar_transform!(
    add_sccp_pass, LLVMAddSCCPPass);
forward_scalar_transform!(
    add_scalar_repl_aggregates_pass, LLVMAddScalarReplAggregatesPass);
forward_scalar_transform!(
    add_scalar_repl_aggregates_pass_ssa, LLVMAddScalarReplAggregatesPassSSA);

use llvm_sys::transforms::scalar::LLVMAddScalarReplAggregatesPassWithThreshold;
impl<'a> FunctionPassManager<'a> {
    pub fn add_scalar_repl_aggregates_pass_with_threshold(&self,
                                                          threshold: ::libc::c_int) {
        unsafe {
            LLVMAddScalarReplAggregatesPassWithThreshold(self.into(), threshold);
        }
    }
}

forward_scalar_transform!(
    add_simplify_lib_calls_pass, LLVMAddSimplifyLibCallsPass);
forward_scalar_transform!(
    add_tail_call_elimination_pass, LLVMAddTailCallEliminationPass);
forward_scalar_transform!(
    add_constant_propagation_pass, LLVMAddConstantPropagationPass);
forward_scalar_transform!(
    add_demote_memory_to_register_pass, LLVMAddDemoteMemoryToRegisterPass);
forward_scalar_transform!(
    add_verify_pass, LLVMAddVerifierPass);
forward_scalar_transform!(
    add_correlated_value_propagation_pass, LLVMAddCorrelatedValuePropagationPass);
forward_scalar_transform!(
    add_early_cse_pass, LLVMAddEarlyCSEPass);
forward_scalar_transform!(
    add_lower_expect_intrinsic_pass, LLVMAddLowerExpectIntrinsicPass);
forward_scalar_transform!(
    add_type_based_alias_analysis_pass, LLVMAddTypeBasedAliasAnalysisPass);
forward_scalar_transform!(
    add_scoped_no_alias_aa_pass, LLVMAddScopedNoAliasAAPass);
forward_scalar_transform!(
    add_basic_alias_analysis_pass, LLVMAddBasicAliasAnalysisPass);

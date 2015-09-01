#include "llvm-c/Analysis.h"
#include "llvm-c/Initialization.h"
#include "llvm/IR/Module.h"
#include "llvm/IR/Verifier.h"
#include "llvm/InitializePasses.h"
#include "llvm/PassRegistry.h"
#include "llvm/Support/raw_ostream.h"
#include <cstring>

using namespace llvm;

extern "C" {
    LLVMBool LLVMVerifyFunctionWithOutput(LLVMValueRef Fn,
                                          LLVMVerifierFailureAction Action,
                                          char **OutMessages) {
        raw_ostream *DebugOS = Action != LLVMReturnStatusAction ? &errs() : nullptr;
        std::string Messages;
        raw_string_ostream MsgsOS(Messages);

        LLVMBool Result = verifyFunction(*unwrap<Function>(Fn), OutMessages ? &MsgsOS : DebugOS);

        // Duplicate the output to stderr.
        if (DebugOS && OutMessages)
            *DebugOS << MsgsOS.str();

        if (Action == LLVMAbortProcessAction && Result)
            report_fatal_error("Broken function found from local wrapper, compilation aborted!");

        if (OutMessages)
            *OutMessages = strdup(MsgsOS.str().c_str());

        return Result;
    }
}

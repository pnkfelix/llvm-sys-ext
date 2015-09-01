LLVM-ext
=======

This is a library that extends [llvm-alt] with more methods that are
provided by the underlying [llvm-sys] library but have not yet been
exposed in the types defined in [llvm-alt].

Using in your projects
----------------------

To use this in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
...
llvm-alt = "*"
llvm-ext = "*"
```

and then in each source module where you want to use one of the
extension methods on a type exported by `llvm-alt`, import the
corresponding extension trait from `llvm-ext`. You will almost
certainly need to rename the trait on import, but since you are only
importing it to bring the trait's methods into scope, you will
not need to refer to the trait by name, and thus can rename it
to anything you like.

For example, to pull in the extension methods on `Function`:

```rust
extern crate llvm_ext;
...
use llvm_ext::Function as FunctionExt;
...
fn compile_function_definition(...) {
    let f: &llvm::Function = try!(lookup(...));
    ...
    if !f.empty() { // <-- `empty` method injected by `FunctionExt`
        return err(CodegenErrorKind::FunctionRedefinition);
    }
    ...
```

[llvm-alt]: https://crates.io/crates/llvm_alt

[llvm-sys]: https://crates.io/crates/llvm_sys

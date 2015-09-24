LLVM-ext
=======

This is a library that extends [llvm-sys] with more methods that are
provided by LLVM but have not yet been exposed in the types defined in
[llvm-sys].

Using in your projects
----------------------

To use this in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
...
llvm-sys-ext = "*"
```

[llvm-sys]: https://crates.io/crates/llvm_sys

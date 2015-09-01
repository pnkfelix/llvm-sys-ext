extern crate gcc;

fn main() {
    // gcc::compile_library("libwrapper.a", &["src/wrapper.cpp"]);
    use std::process::Command;
    let incl = Command::new("llvm-config").arg("--includedir").output().unwrap();
    let incl = String::from_utf8(incl.stdout).unwrap();
    gcc::Config::new()
        .file("src/wrapper.cpp")
        .include(incl)
        .define("__STDC_CONSTANT_MACROS", None)
        .define("__STDC_LIMIT_MACROS", None)
        .flag("-std=c++11")
        .cpp(true)
        .compile("libwrapper.a");
    // gcc::compile_library("libwrapper.a", &["src/wrapper.cpp"]);
}

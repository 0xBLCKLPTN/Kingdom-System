visibility = ["//visibility:public"]

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")

rust_binary(
    name = "rs_bazel",
    
    # Specifies the source file for the binary.
    srcs = glob([
        "src/*.rs",
        "src/**/*.rs"
    ]),
    aliases = aliases(),
    #proc_macro_deps = all_crate_deps(),
    deps = all_crate_deps(normal = True),
    # Specifies the Rust edition to use for this binary.
    edition = "2021"
)

rust_test(
    name = "rs_test",
    
    # Specifies the source file for the binary.
    srcs = glob([
        "src/*.rs",
        "src/log_db/**/*.rs",
        "src/remore_db/**/*.rs",
    ]),
    aliases = aliases(),
    #proc_macro_deps = all_crate_deps(),
    deps = all_crate_deps(normal = True),
    # Specifies the Rust edition to use for this binary.
    edition = "2021"
)
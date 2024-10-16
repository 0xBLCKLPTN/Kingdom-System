visibility = ["//visibility:public"]

load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test", "rust_library")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")


rust_library(
    name = "alice_db_lib",

    # Specifies the source file for the binary.
    srcs = glob([
        "src/*.rs",
        "src/**/*.rs",
    ]),
    aliases = aliases(),
    #proc_macro_deps = all_crate_deps(),
    #deps = [
    #    all_crate_deps(normal=True)
    #],
    deps = all_crate_deps(normal = True),
    # Specifies the Rust edition to use for this binary.
    edition = "2021"
)

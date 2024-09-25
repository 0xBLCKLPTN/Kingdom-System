load("@rules_rust//rust:toolchain.bzl", "rust_toolchain")

rust_toolchain(
    name = "rust_toolchain",
    edition = "2021",  # or "2021" depending on your project
    target_cpu = "x86_64",
    target_os = "macos",  # or "macos", "windows", etc.
)
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    integrity = "sha256-Weev1uz2QztBlDA88JX6A1N72SucD1V8lBsaliM0TTg=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.48.0/rules_rust-v0.48.0.tar.gz"],
)


load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
rules_rust_dependencies()
rust_register_toolchains()

load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")
rust_analyzer_dependencies()

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "crate")
crates_repository(
    name = "crate_index",
    cargo_lockfile = "//Plugins/AliceDatabase:Cargo.lock",
    lockfile = "//Plugins/AliceDatabase:Cargo.Bazel.lock",
    manifests = [
        "//Plugins/AliceDatabase:Cargo.toml",
    ],
    # Should match the version represented by the currently registered `rust_toolchain`.
    rust_version = "1.81.0",
)

load("@crate_index//:defs.bzl", "crate_repositories")
crate_repositories()

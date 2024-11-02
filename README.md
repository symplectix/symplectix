## Go

To (re-)generate BUILD files:

    bazel run //:gazelle

`bazel mod tidy`:

    bazel mod tidy

`go mod tidy`:

    bazel run @rules_go//go -- mod tidy

## Rust

To (re-)generate the `rust-project.json`:

    bazel run @rules_rust//tools/rust_analyzer:gen_rust_project

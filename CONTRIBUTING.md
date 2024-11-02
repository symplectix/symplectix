# Contributing to symplectix

## Rust

> [!tip] To (re-)generate the rust-project.json
>
>     bazel run @rules_rust//tools/rust_analyzer:gen_rust_project

## Go

> [!tip] To (re-)generate BUILD files
>
>     bazel run //:gazelle

> [!tip] Managing dependencies
>
>     bazel run @rules_go//go -- mod tidy

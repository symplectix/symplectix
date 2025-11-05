# Contributing to syx

## Rust

> [!tip] To (re-)generate the rust-project.json
>
>     tools/gen-rust-project

## Go

> [!tip] To (re-)generate BUILD files
>
>     bazel run //:gazelle

> [!tip] Managing dependencies
>
>     bazel run @rules_go//go -- mod tidy

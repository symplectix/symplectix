#!/usr/bin/env bash

cd "$BUILD_WORKSPACE_DIRECTORY"
exec bazel run @rules_rust//tools/rust_analyzer:gen_rust_project

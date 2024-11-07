#!/usr/bin/env bash
exec bazel run "@multitool//tools/$( basename $0 ):cwd" -- "$@"

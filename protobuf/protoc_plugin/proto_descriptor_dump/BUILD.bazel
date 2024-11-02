load("@rules_proto_grpc//:defs.bzl", "proto_plugin")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")

rust_library(
    name = "proto_descriptor_dump",
    srcs = ["lib.rs"],
    deps = [
        "//protobuf/protoc_plugin",
        "@crates//:prost",
        "@crates//:prost-reflect",
        "@crates//:prost-types",
    ],
)

rust_binary(
    name = "gen_message_descriptor_dump",
    srcs = ["gen_message_descriptor_dump.rs"],
    rustc_flags = [
        "-Copt-level=3",
        "-Cdebuginfo=2",
    ],
    deps = [
        ":proto_descriptor_dump",
        "//protobuf/protoc_plugin",
        "@crates//:anyhow",
    ],
)

proto_plugin(
    name = "message_descriptor_dump",
    outputs = ["{protopath}.message_descriptor_dump"],
    tool = ":gen_message_descriptor_dump",
    visibility = ["//visibility:public"],
)

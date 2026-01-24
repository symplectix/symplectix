load("//bazel:proto.bzl", "proto")
load("//bazel:rust.bzl", "rust")

proto.plugin(
    name = "message_descriptor_dump",
    outputs = ["{protopath}.message_descriptor_dump"],
    tool = ":protoc_gen_message_descriptor_dump",
    visibility = ["//:__subpackages__"],
)

rust.binary(
    name = "protoc_gen_message_descriptor_dump",
    srcs = ["message_descriptor_dump.rs"],
    binary_name = "protoc-gen-message-descriptor-dump",
    deps = [
        "//proto_plugin",
        "@crates//:anyhow",
        "@crates//:prost-reflect",
        "@crates//:prost-types",
    ],
)

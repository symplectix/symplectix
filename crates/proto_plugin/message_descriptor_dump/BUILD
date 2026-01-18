load("//bazel:proto.bzl", "proto")
load("//bazel:rust.bzl", "rust")

package(default_visibility = ["//:__subpackages__"])

rust.binary(
    name = "protoc_gen_message_descriptor_dump",
    srcs = ["message_descriptor_dump.rs"],
    binary_name = "protoc-gen-message-descriptor-dump",
    deps = [
        "//crates/proto_plugin",
        "@crates//:anyhow",
        "@crates//:prost-reflect",
        "@crates//:prost-types",
    ],
)

proto.plugin(
    name = "message_descriptor_dump",
    outputs = ["{protopath}.message_descriptor_dump"],
    tool = ":protoc_gen_message_descriptor_dump",
)

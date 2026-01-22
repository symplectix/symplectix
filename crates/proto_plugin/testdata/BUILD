load("//build:proto.bzl", "proto")
load("//crates/proto_plugin/message_descriptor_dump:defs.bzl", "message_descriptor_dump")

proto.library(
    name = "example_proto",
    srcs = [
        "example.proto",
        "service.proto",
    ],
    deps = [
        "@protobuf//:any_proto",
        "@protobuf//:empty_proto",
    ],
)

proto.library(
    name = "service_proto",
    srcs = [
        "service.proto",
    ],
    deps = [
        ":example_proto",
    ],
)

message_descriptor_dump(
    name = "testdata_message_descriptor_dump",
    protos = [
        "service_proto",
        ":example_proto",
    ],
)

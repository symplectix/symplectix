load("@protobuf//bazel:proto_library.bzl", "proto_library")
load("@rules_proto_grpc//:defs.bzl", "proto_plugin")

proto = struct(
    library = proto_library,
    plugin = proto_plugin,
)

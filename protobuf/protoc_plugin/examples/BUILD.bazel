load("@rules_proto//proto:defs.bzl", "proto_library")
load("//protobuf/protoc_plugin:defs.bzl", "message_descriptor_dump_compile")

proto_library(
    name = "customer_proto",
    srcs = ["customer.proto"],
    deps = [
        ":product_proto",
        "@com_google_protobuf//:any_proto",
    ],
)

proto_library(
    name = "product_proto",
    srcs = ["product.proto"],
    deps = ["@com_google_protobuf//:empty_proto"],
)

message_descriptor_dump_compile(
    name = "examples_message_descriptor_dump",
    protos = [
        ":customer_proto",
        ":product_proto",
    ],
)

load(
    "@rules_proto_grpc//:defs.bzl",
    "ProtoPluginInfo",
    "proto_compile_attrs",
    "proto_compile_impl",
)

empty_file_compile = rule(
    implementation = proto_compile_impl,
    attrs = dict(
        proto_compile_attrs,
        # List of protoc plugins to apply
        _plugins = attr.label_list(
            providers = [ProtoPluginInfo],
            default = [
                Label("//protobuf/protoc_plugin/empty_file"),
            ],
        ),
    ),
    toolchains = [str(Label("@rules_proto_grpc//protobuf:toolchain_type"))],
)

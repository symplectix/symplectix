alias(
    name = "target-determinator_bin",
    actual = select({
        "//platforms:linux_x86_64": "@target-determinator_linux_x86_64//file",
        "//platforms:macos_x86_64": "@target-determinator_macos_x86_64//file",
        "//platforms:macos_arm64": "@target-determinator_macos_arm64//file",
    }),
)

alias(
    name = "target-determinator-driver_bin",
    actual = select({
        "//platforms:linux_x86_64": "@target-determinator-driver_linux_x86_64//file",
        "//platforms:macos_x86_64": "@target-determinator-driver_macos_x86_64//file",
        "//platforms:macos_arm64": "@target-determinator-driver_macos_arm64//file",
    }),
)

sh_binary(
    name = "target-determinator",
    srcs = ["exec.sh"],
    args = ["$(location :target-determinator_bin)"],
    data = [":target-determinator_bin"],
)

sh_binary(
    name = "driver",
    srcs = ["exec.sh"],
    args = ["$(location :target-determinator-driver_bin)"],
    data = [":target-determinator-driver_bin"],
)

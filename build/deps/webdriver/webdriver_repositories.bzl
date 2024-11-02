"""Repositories for webdrivers"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def geckodriver_repositories():
    maybe(
        http_archive,
        name = "geckodriver_linux_x86_64",
        build_file = Label("@//build/deps/webdriver:BUILD.geckodriver.bazel"),
        sha256 = "5f5e89bb31fe5f55f963f56ef7e55a5c8e9dc415d94b1ddc539171a327b8e6c4",
        url = "https://github.com/mozilla/geckodriver/releases/download/v0.33.0/geckodriver-v0.33.0-linux64.tar.gz",
    )

    maybe(
        http_archive,
        name = "geckodriver_macos_arm64",
        build_file = Label("@//build/deps/webdriver:BUILD.geckodriver.bazel"),
        sha256 = "36ec6d2ff40d4019ac348bd96b83be46917cc8c3e87184890c3de995890d8e2c",
        url = "https://github.com/mozilla/geckodriver/releases/download/v0.33.0/geckodriver-v0.33.0-macos-aarch64.tar.gz",
    )

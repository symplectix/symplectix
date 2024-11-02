load("//build/deps/libgit2:libgit2_repositories.bzl", "libgit2_repositories")
load("//build/deps/libssh2:libssh2_repositories.bzl", "libssh2_repositories")
load("//build/deps/openssl:openssl_repositories.bzl", "openssl_repositories")
load("//build/deps/zlib:zlib_repositories.bzl", "zlib_repositories")

def repositories():
    openssl_repositories()
    zlib_repositories()
    libssh2_repositories()
    libgit2_repositories()

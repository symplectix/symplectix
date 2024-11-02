load("//thirdparty/crates:crates_repositories.bzl", "crates_repositories")
load("//thirdparty/libgit2:libgit2_repositories.bzl", "libgit2_repositories")
load("//thirdparty/libssh2:libssh2_repositories.bzl", "libssh2_repositories")
load("//thirdparty/openssl:openssl_repositories.bzl", "openssl_repositories")
load("//thirdparty/zlib:zlib_repositories.bzl", "zlib_repositories")

def repositories():
    openssl_repositories()
    zlib_repositories()
    libssh2_repositories()
    libgit2_repositories()
    crates_repositories()

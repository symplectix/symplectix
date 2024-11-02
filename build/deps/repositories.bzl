load("//build/deps/openssl:openssl_repositories.bzl", "openssl_repositories")
load("//build/deps/webdriver:webdriver_repositories.bzl", "geckodriver_repositories")
load("//build/deps/zlib:zlib_repositories.bzl", "zlib_repositories")

def repositories():
    openssl_repositories()
    zlib_repositories()
    geckodriver_repositories()

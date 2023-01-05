from build import *


class Curl(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "An URL retrieval utility and library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "pkgconf/[^1.7.3]",
    )
    requires = (
        "zlib/[^1.2.11]",
        "libssl/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://curl.haxx.se/download/curl-{self.version}.tar.gz")

    def build(self):
        os.environ["CFLAGS"] += "-ldl -lpthread"
        self.autotools()

    def package_info(self):
        self.env_info.CURL_CA_BUNDLE = os.path.join(
            self.deps_cpp_info["ca-certificates"].rootpath, "etc", "ssl", "certs", "cert.pem"
        )

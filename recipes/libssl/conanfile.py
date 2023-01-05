from build import *


class Libssl(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Virtual libssl"
    license = "MIT"
    options = {}
    default_options = {}
    requires = (
        "openssl/[^3.0.0-alpha7]",
        "ca-certificates/[^20191127]",
    )

    def package_info(self):
        self.env_info.SSL_CERT_DIR = os.path.join(
            self.deps_cpp_info["ca-certificates"].rootpath, "etc", "ssl", "certs"
        )
        self.env_info.SSL_CERT_FILE = os.path.join(
            self.deps_cpp_info["ca-certificates"].rootpath,
            "etc",
            "ssl",
            "certs",
            "ca-certificates.crt",
        )

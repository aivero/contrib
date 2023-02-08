from build import *

CONFIG_MAK = """
PROFILE=
CFLAGS={}
LDFLAGS=-ldl
INSTALL_SYMLINKS=1
"""


class Git(Recipe):
    description = "The fast distributed version control system"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "gettext/[^0.21]",
    )
    requires = (
        "curl/[^7.73.0]",
        "expat/[^2.2.7]",
    )

    def source(self):
        self.get(f"https://www.kernel.org/pub/software/scm/git/git-{self.version}.tar.xz")

    def build(self):
        with open(os.path.join(self.src, "config.mak"), "w") as cfg:
            cfg.write(CONFIG_MAK.format(f"-I{self.source_folder} {os.environ['CFLAGS']}"))

        args = [
            f"prefix={self.package_folder}",
        ]
        self.make(args, target="all")
        self.make(args, target="install")

    def package_info(self):
        self.env_info.GIT_CONFIG_SYSTEM = "/etc/gitconfig"
        self.env_info.GIT_EXEC_PATH = os.path.join(self.package_folder, "libexec", "git-core")
        self.env_info.GIT_SSL_CAINFO = os.path.join(
            self.deps_cpp_info["ca-certificates"].rootpath, "etc", "ssl", "certs", "cert.pem"
        )

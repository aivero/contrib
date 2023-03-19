from build import *


class LibgpgError(Recipe):
    description = "Support library for libgcrypt"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://www.gnupg.org/ftp/gcrypt/libgpg-error/libgpg-error-{self.version}.tar.bz2")

    def build(self):
        args = ["--enable-install-gpg-error-config"]
        self.autotools(args)

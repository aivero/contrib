from build import *


class Libgcrypt(Recipe):
    description = "General purpose cryptographic library based on the code from GnuPG"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = (
        "libgpg-error/[^1.46]",
    )

    def source(self):
        self.get(f"https://gnupg.org/ftp/gcrypt/libgcrypt/libgcrypt-{self.version}.tar.bz2")

    def build(self):
        args = [f"--with-gpg-error-prefix={self.deps_cpp_info['libgpg-error'].rootpath}", "--disable-asm"]
        self.autotools(args)

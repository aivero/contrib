from build import *


class Libtool(Recipe):
    description = "A generic library support script"
    license = "GPL"
    exports = "libtool-prefix-fix.patch"
    build_requires = (
        "cc/[^1.0.0]",
        "automake/[^1.16.1]",
        "help2man/[^1.47.11]",
        "texinfo/[^6.6]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/libtool/libtool-{self.version}.tar.gz")
        self.patch("libtool-prefix-fix.patch")

    def package_info(self):
        self.env_info.LIBTOOL_PREFIX = self.package_folder
        self.env_info.LIBTOOL = os.path.join(self.package_folder, "bin", "libtool")
        self.env_info.LIBTOOLIZE = os.path.join(self.package_folder, "bin", "libtoolize")
        self.env_info.ACLOCAL_PATH.append(os.path.join(self.package_folder, "share", "aclocal"))

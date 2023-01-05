from build import *


class Autoconf(Recipe):
    description = "A GNU tool for automatically configuring source code"
    license = "GPL"
    exports = "m4-include.patch"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )
    requires = (
        "make/[^4.3]",
        "m4/[^1.4.18]",
        "perl/[^5.30.0]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/autoconf/autoconf-{self.version}.tar.gz")
        self.patch("m4-include.patch")

    def package_info(self):
        self.env_info.AUTOCONF = os.path.join(self.package_folder, "bin", "autoconf")
        self.env_info.AUTOHEADER = os.path.join(self.package_folder, "bin", "autoheader")
        self.env_info.AUTOM4TE = os.path.join(self.package_folder, "bin", "autom4te")
        self.env_info.AC_MACRODIR = os.path.join(self.package_folder, "share", "autoconf")
        self.env_info.autom4te_perllibdir = os.path.join(self.package_folder, "share", "autoconf")

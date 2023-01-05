from build import *


class Automake(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "A GNU tool for automatically creating Makefiles"
    license = "GPL"
    exports = "automake-include-fix.patch"
    requires = ("autoconf/[^2.69]",)

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/automake/automake-{self.version}.tar.gz")
        self.patch(f"automake-include-fix.patch")

    def package_info(self):
        self.env_info.AUTOMAKE = os.path.join(self.package_folder, "bin", "automake")
        self.env_info.AUTOMAKE_DIR = os.path.join(self.package_folder, "share")
        self.env_info.AUTOMAKE_LIBDIR = os.path.join(self.package_folder, "share", "automake-1.16")
        self.env_info.ACLOCAL = os.path.join(self.package_folder, "bin", "aclocal")
        self.env_info.ACLOCAL_DIR = os.path.join(self.package_folder, "share")
        self.env_info.ACLOCAL_PATH.append(
            os.path.join(self.package_folder, "share", "aclocal-1.16")
        )
        self.env_info.PERL5LIB.append(os.path.join(self.package_folder, "share", "automake-1.16"))

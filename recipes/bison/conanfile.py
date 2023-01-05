from build import *


class Bison(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Bison is a general-purpose parser generator"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )
    requires = ("m4/[^1.4.18]",)

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/bison/bison-{self.version}.tar.gz")

    def package_info(self):
        self.env_info.BISON_PKGDATADIR = os.path.join(self.package_folder, "share", "bison")

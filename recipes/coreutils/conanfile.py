from build import *


class Coreutils(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = (
        "The basic file, shell and text manipulation utilities of the GNU operating system"
    )
    license = "GPL"

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/coreutils/coreutils-{self.version}.tar.xz")

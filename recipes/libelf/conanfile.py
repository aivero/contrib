from build import *


class Libelf(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Handle ELF object files and DWARF debugging information"
    license = "LGPL"
    requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")

    def source(self):
        self.get(f"https://fossies.org/linux/misc/old/libelf-{self.version}.tar.gz")
        self.download(
            f"https://git.savannah.gnu.org/gitweb/?p=config.git;a=blob_plain;f=config.guess;hb=HEAD",
            "config.guess",
        )

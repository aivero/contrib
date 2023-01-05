from build import *


class Make(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "GNU make utility to maintain groups of programs"
    license = "GPL"
    build_requires = "cc/[^1.0.0]"

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/make/make-{self.version}.tar.gz")

    def package_info(self):
        self.env_info.MAKE = os.path.join(self.package_folder, "bin", "make")

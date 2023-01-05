from build import *


class MesonRecipe(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "High productivity build system"
    license = "Apache"
    requires = (
        "python-setuptools/[^50.3.2]",
        "cc/[^1.0.0]",
        "ninja/[^1.9.0]",
        "pkgconf/[^1.7.3]",
    )

    def source(self):
        self.get(
            f"https://github.com/mesonbuild/meson/releases/download/{self.version}/meson-{self.version}.tar.gz"
        )

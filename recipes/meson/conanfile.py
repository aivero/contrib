from build import *


class MesonRecipe(PythonRecipe):
    description = "High productivity build system"
    license = "Apache"
    requires = (
        "python-setuptools/[^67.3]",
        "cc/[^1.0.0]",
        "ninja/[^1.9.0]",
        "pkgconf/[^1.7.3]",
    )

    def source(self):
        self.get(
            f"https://github.com/mesonbuild/meson/releases/download/{self.version}/meson-{self.version}.tar.gz"
        )

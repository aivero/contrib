from build import *


class PythonPillowRecipe(PythonRecipe):
    description = "Python Image Library"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "pkgconf/[^1.6.3]",
        "python-setuptools/[^67.3]",
        "zlib/[^1.2.11]",
    )
    requires = ("openjpeg2/[^2.4.0]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/python-pillow/Pillow/archive/{self.version}.tar.gz")

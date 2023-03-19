from build import *


class Libplist(PythonRecipe):
    description = "A library to handle Apple Property List format whereas it's binary or XML"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "cython/[^0.29.33]",
        "autoconf-archive/[^2021.02.19]",
    )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/libimobiledevice/libplist/archive/refs/tags/{self.version}.tar.gz")

    def build(self):
        args = []
        self.autotools(args)

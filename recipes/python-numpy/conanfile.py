from build import *


class PythonNumpyRecipe(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "conan package for Python Numpy module"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "pkgconf/[^1.6.3]",
    )
    requires = (
        "python-setuptools/[>=40.4.0]",
        "python-cython/[^0.29.19]",
    )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(
            f"https://github.com/numpy/numpy/releases/download/v{self.version}/numpy-{self.version}.tar.gz"
        )

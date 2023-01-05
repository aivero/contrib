from build import *


class PythonCairoRecipe(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "Python bindings for the cairo graphics library"
    license = "LGPL"
    build_requires = ("cc/[^1.0.0]", "meson/[>=0.55.3]")
    requires = ("cairo/[^1.16.0]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(
            f"https://github.com/pygobject/pycairo/releases/download/v{self.version}/pycairo-{self.version}.tar.gz"
        )

from build import *


class PythonDistlibRecipe(PythonRecipe):
    description = "Low-level components of distutils2/packaging"
    license = "PSF"

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(
            f"https://files.pythonhosted.org/packages/source/d/distlib/distlib-{self.version}.zip"
        )

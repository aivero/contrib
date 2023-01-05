from build import *


class PythonPipRecipe(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "High productivity build system"
    license = "MIT"
    build_requires = ("python-setuptools/[>=41.2.0]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/pypa/pip/archive/{self.version}.tar.gz")

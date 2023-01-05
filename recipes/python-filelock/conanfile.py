from build import *


class PythonFilelockRecipe(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "A platform independent file lock"
    license = "custom"
    requires = ("python-setuptools/[>=41.2.0]",)

    def source(self):
        self.get(f"https://github.com/benediktschmitt/py-filelock/archive/v{self.version}.tar.gz")

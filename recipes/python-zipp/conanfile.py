from build import *


class PythonZippRecipe(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "Pathlib-compatible object wrapper for zip files"
    license = "MIT"
    requires = ("python-setuptools/[^50.3.2]",)

    def source(self):
        self.get(f"https://github.com/jaraco/zipp/archive/v{self.version}.tar.gz")

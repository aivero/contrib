from build import *


class PythonImportlibMetadataRecipe(PythonRecipe):
    description = "Read metadata from Python packages"
    license = "Apache"
    requires = ("python-zipp/[^3.1.0]",)

    def source(self):
        self.get(
            f"https://github.com/python/importlib_metadata/archive/refs/tags/v{self.version}.tar.gz"
        )

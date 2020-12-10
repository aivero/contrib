from build import *


class PythonSetuptoolsRecipe(PythonRecipe):
    description = "Easily download, build, install, upgrade, and uninstall Python packages"
    license = "Apache"
    requires = ("python/[^3.8.5]",)

    def source(self):
        self.get(f"https://github.com/pypa/setuptools/archive/v{self.version}.tar.gz")

    def build(self):
        self.exe("python bootstrap.py")
        self.setuptools()

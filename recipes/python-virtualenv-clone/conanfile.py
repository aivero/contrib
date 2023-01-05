from build import *


class PythonVirtualenvClone(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "A script for cloning a non-relocatable virtualenv."
    license = "GPL"
    requires = ("python-setuptools/[>=40]",)

    def source(self):
        self.get(
            f"https://files.pythonhosted.org/packages/source/v/virtualenv-clone/virtualenv-clone-{self.version}.tar.gz"
        )

    def build(self):
        self.setuptools()

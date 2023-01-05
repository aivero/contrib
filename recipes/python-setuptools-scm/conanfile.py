from build import *


class PythonSetuptoolsScm(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "Handles managing your python package versions in scm metadata"
    license = "MIT"
    requires = ("python-setuptools/[>=40.4.0]",)

    def source(self):
        self.get(
            f"https://files.pythonhosted.org/packages/source/s/setuptools_scm/setuptools_scm-{self.version}.tar.gz"
        )

from build import *


class PythonCertifi(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "Python package for providing Mozilla's CA Bundle"
    license = "GPL"
    requires = ("python-setuptools/[>=40]",)

    def source(self):
        self.get(f"https://pypi.io/packages/source/c/certifi/certifi-{self.version}.tar.gz")

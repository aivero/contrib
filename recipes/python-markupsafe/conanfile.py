from build import *


class PythonMarkupsafe(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "Implements a XML/HTML/XHTML Markup safe string for Python"
    license = "BSD"
    build_requires = ("python-setuptools/[>=40.4.0]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/pallets/markupsafe/archive/{self.version}.tar.gz")

from build import *


class PythonDocutils(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = (
        "Set of tools for processing plaintext docs into formats such as HTML, XML, or LaTeX"
    )
    license = "custom"
    build_requires = ("python-setuptools/[>=41.2.0]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://downloads.sourceforge.net/docutils/docutils-{self.version}.tar.gz")

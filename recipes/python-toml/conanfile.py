from build import *


class PythonToml(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "A Python library for parsing and creating TOML"
    license = "MIT"
    build_requires = ("python-setuptools/[>=40.4.0]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(
            f"https://files.pythonhosted.org/packages/source/t/toml/toml-{self.version}.tar.gz"
        )

from build import *


class Pipenv(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "Sacred Marriage of Pipfile, Pip, & Virtualenv."
    license = "MIT"
    requires = (
        "python-pip/[>=19.2.3]",
        "python-virtualenv/[>=20.0.21]",
        "python-virtualenv-clone/[>=0.5.6]",
        "python-certifi/[^2020.12.5]",
    )

    def source(self):
        self.get(f"https://github.com/pypa/pipenv/archive/refs/tags/v{self.version}.tar.gz")

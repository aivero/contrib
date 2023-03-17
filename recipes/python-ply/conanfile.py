from build import *


class PythonPly(PythonRecipe):
    description = "Lex Yacc for Python"
    license = "BSD"
    build_requires = (
        "python-setuptools/[>=40.4.0]",
        "python-pip/[>=20.3.4]",
        )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get("https://files.pythonhosted.org/packages/e5/69/882ee5c9d017149285cab114ebeab373308ef0f874fcdac9beb90e0ac4da/ply-3.11.tar.gz")

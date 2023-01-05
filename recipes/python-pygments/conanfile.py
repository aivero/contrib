from build import *


class PythonPygments(PythonRecipe):
    description = "Python syntax highlighter"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = ("python-setuptools/[^50.3.2]",)

    def source(self):
        self.get(f"https://pypi.org/packages/source/P/Pygments/Pygments-{self.version}.tar.gz")

from build import *


class PythonSixRecipe(PythonRecipe):
    description = "Python 2 and 3 compatibility utilities"
    license = "MIT"
    build_requires = (
        "autotools/1.0.0",
        "python-setuptools/[>=41.2.0]",
    )

    def source(self):
        self.get(f"https://pypi.io/packages/source/s/six/six-{self.version}.tar.gz")

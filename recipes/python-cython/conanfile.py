from build import *


class PythonCython(Recipe):
    description = "Python to C compiler"
    license = "Apache"
    settings = "build_type", "python"
    build_requires = (
        "cc/[^1.0.0]",
        "pkgconf/[^1.6.3]",
    )
    requires = ("python-setuptools/[>=40.4.0]",)

    def source(self):
        self.get(f"https://github.com/cython/cython/archive/{self.version}.tar.gz")

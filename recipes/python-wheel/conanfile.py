from build import *


class PythonWheel(PythonRecipe):
    description = "A built-package format for Python"
    license = "MIT"
    requires = (
        "python-setuptools/[>=50.3.2]",    
    )

    def source(self):
        self.get(f"https://github.com/pypa/wheel/archive/{self.version}.tar.gz")
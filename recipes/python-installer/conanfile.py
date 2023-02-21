from build import *


class PythonInstaller(PipRecipe):
    description = "Low-level library for installing a Python package from a wheel distribution"
    license = "MIT"
    build_requires = (
        "python-pip/[>=20.3.4]",
    )
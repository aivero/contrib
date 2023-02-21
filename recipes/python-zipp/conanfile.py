from build import *


class PythonZippRecipe(PipRecipe):
    description = "Pathlib-compatible object wrapper for zip files"
    license = "MIT"
    requires = ("python-setuptools/[^67.3.2]",)
    build_requires = (
        "python-pip/[>=20.3.4]",
    )
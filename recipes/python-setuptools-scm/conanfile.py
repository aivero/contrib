from build import *


class PythonSetuptoolsScm(PipRecipe):
    description = "Handles managing your python package versions in scm metadata"
    license = "MIT"
    requires = (
        "python-setuptools/[>=40.4.0]",
        "python-tomli/[^2.0.1]",
    )
    build_requires = (
        "python-pip/[>=20.3.4]",
    )
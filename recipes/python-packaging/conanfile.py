from build import *


class PythonPackaging(PipRecipe):
    description = "Core utilities for Python packages"
    license = "Apache"
    build_requires = (
        "python-pip/[>=20.3.4]",
    )
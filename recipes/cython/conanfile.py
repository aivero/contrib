from build import *


class Cython(PipRecipe):
    description = "C-Extensions for Python"
    license = "Apache"
    build_requires = (
        "python-pip/[>=20.3.4]",
    )
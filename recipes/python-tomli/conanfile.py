from build import *


class PythonTomli(PipRecipe):
    description = "A lil' TOML parser"
    license = "MIT"
    build_requires = (
        "python-pip/[>=20.3.4]",
    )
from build import *


class PythonToml(PipRecipe):
    description = "A Python library for parsing and creating TOML"
    license = "MIT"
    build_requires = (
        "python-pip/[>=20.3.4]",
    )
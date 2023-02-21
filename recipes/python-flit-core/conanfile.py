from build import *


class PythonFlitCore(PipRecipe):
    description = "A PEP 517 build backend for packages using Flit"
    license = "BSD"
    build_requires = (
        "python-pip/[>=20.3.4]",
    )
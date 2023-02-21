from build import *


class PythonMarkupsafe(PipRecipe):
    description = "Implements a XML/HTML/XHTML Markup safe string for Python"
    license = "BSD"
    build_requires = (
        "python-pip/[>=20.3.4]",
    )
from build import *


class PythonPyprojectHooks(PipRecipe):
    description = "A low-level library for calling build-backends in pyproject.toml-based project"
    license = "MIT"
    requires = ("python-setuptools/[>=50.3.2]",)
    build_requires = (
        "python-pip/[>=20.3.4]",
    )
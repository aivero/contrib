from build import *


class PythonBuild(PipRecipe):
    description = "A simple, correct PEP 517 build frontend"
    license = "MIT"
    build_requires = (
        "python-pip/[>=20.3.4]",
    )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

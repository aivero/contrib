from build import *


class PythonVirtualenvRecipe(PipRecipe):
    description = "Virtual Python Environment builder"
    license = "MIT"

    build_requires = (
        "python-pip/[>=20.3.4]",
    )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")
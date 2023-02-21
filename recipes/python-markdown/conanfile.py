from build import *


class PythonMarkdown(PipRecipe):
    description = "Python implementation of John Gruber's Markdown."
    license = "BSD"
    build_requires = (
        "python-pip/[>=20.3.4]",
    )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")
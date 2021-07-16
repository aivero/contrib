from build import *


class PythonAppdirsRecipe(Recipe):
    description = 'A small Python module for determining appropriate platform-specific dirs, e.g. a "user data dir".'
    license = "MIT"
    settings = "build_type", "compiler",    "python"
    build_requires = ("python-setuptools/[>=41.2.0]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://pypi.io/packages/source/a/appdirs/appdirs-{self.version}.tar.gz")

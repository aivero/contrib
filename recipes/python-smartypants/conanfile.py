from build import *


class PythonSmartypants(PythonRecipe):
    description = "Python with the SmartyPants"
    license = "BSD"

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/leohemsted/smartypants.py/archive/v{self.version}.tar.gz")

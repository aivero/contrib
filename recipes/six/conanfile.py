from build import *


class Six(PythonRecipe):
    description = "Six"
    license = "custom"
    def build_requirements(self):
        self.build_requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://files.pythonhosted.org/packages/71/39/171f1c67cd00715f190ba0b100d606d440a28c93c7714febeca8b79af85e/six-{self.version}.tar.gz")

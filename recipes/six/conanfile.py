from build import *


class Six(PythonRecipe):
    description = "Six"
    license = "custom"
    build_requires = (
        "python/[^3.8.12]",
    )


    def source(self):
        self.get(f"https://files.pythonhosted.org/packages/71/39/171f1c67cd00715f190ba0b100d606d440a28c93c7714febeca8b79af85e/six-{self.version}.tar.gz")

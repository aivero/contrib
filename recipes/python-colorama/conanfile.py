from build import *


class Colorama(PythonRecipe):
    description = "Colorama"
    license = "custom"

    build_requires = (
        "python/[^3.8.12]",
    )

    def source(self):
        # Update this to 0.4.5
        self.get(f"https://files.pythonhosted.org/packages/2b/65/24d033a9325ce42ccbfa3ca2d0866c7e89cc68e5b9d92ecaba9feef631df/colorama-{self.version}.tar.gz")

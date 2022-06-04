from build import *


class PythonVirtualenvRecipe(PythonRecipe):
    description = "Virtual Python Environment builder"
    license = "MIT"
    requires = (
        "python-appdirs/[^1.4.4]",
        "python-distlib/[^0.3.0]",
        "python-filelock/[^3.0.12]",
        "python-six/[^1.15.0]",
        "python-importlib-metadata/[>=1.6.0]",
    )

    def source(self):
        self.get(
            f"https://github.com/pypa/virtualenv/archive/{self.version}.tar.gz"
        )

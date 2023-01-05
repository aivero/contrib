from build import *


class PythonJinja(PythonRecipe):
    description = "A simple pythonic template language written in Python"
    license = "BSD"
    requires = (
        "python-setuptools/[>=40.4.0]",
        "python-markupsafe/[^2.0.1]",
    )

    def source(self):
        self.get(
            f"https://files.pythonhosted.org/packages/source/J/Jinja2/Jinja2-{self.version}.tar.gz"
        )

    def build(self):
        self.setuptools()

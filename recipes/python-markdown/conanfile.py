from build import *


class PythonMarkdown(PythonRecipe):
    description = "Python implementation of John Gruber's Markdown."
    license = "BSD"
    build_requires = ("python-setuptools/[>=40.1.0]",)
    requires = ("python-importlib-metadata/[>=1.6.0]",)

    def source(self):
        self.get(
            f"https://files.pythonhosted.org/packages/source/M/Markdown/Markdown-{self.version}.tar.gz"
        )

from build import *


class PyYaml(PythonRecipe):
    description = "Yaml parser for Python"
    license = "MIT"

    build_requires = ("python-setuptools/[>=40.4.0]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")


    def source(self):
        self.get(f"http://pyyaml.org/download/pyyaml/PyYAML-{self.version}.tar.gz")

from build import *


class PythonYaml(PythonRecipe):
    description = "Python bindings for YAML, using fast libYAML library"
    license = "MIT"
    requires = (
        "python-setuptools/[>=50.3.2]",    
        "libyaml/[^0.2.5]",
    )

    def source(self):
        self.get(f"https://files.pythonhosted.org/packages/source/P/PyYAML/PyYAML-{self.version}.tar.gz")

    def build(self):
        self.setuptools()

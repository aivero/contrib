from build import *


class PythonMakoRecipe(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "A super-fast templating language that borrows the best ideas from the existing templating languages"
    license = "Apache"
    build_requires = ("python-setuptools/[^50.3.0]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(
            f"https://github.com/sqlalchemy/mako/archive/rel_{self.version.replace('.', '_')}.tar.gz"
        )

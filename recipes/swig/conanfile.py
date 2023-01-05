from build import *


class SwigRecipe(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "Generate scripting interfaces to C/C++ code"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "perl/[^5.30.0]",
    )

    def build_requirements(self):
        self.build_requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://downloads.sourceforge.net/swig/swig-{self.version}.tar.gz")

    def build(self):
        args = [
            "--without-pcre",
        ]
        self.autotools(args)

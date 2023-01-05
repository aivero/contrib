from build import *


class PythonTypogrify(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "filters to make caring about typography on the web a bit easier"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "python-setuptools/[>=41.2.0]",
    )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(
            f"https://pypi.python.org/packages/source/t/typogrify/typogrify-{self.version}.tar.gz"
        )

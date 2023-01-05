from build import *


class Libffi(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Portable foreign function interface library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )

    def source(self):
        self.get(
            f"https://github.com/libffi/libffi/releases/download/v{self.version}/libffi-{self.version}.tar.gz"
        )

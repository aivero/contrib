from build import *


class LibpthreadStubs(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Library with weak aliases for pthread functions"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://xcb.freedesktop.org/dist/libpthread-stubs-{self.version}.tar.bz2")

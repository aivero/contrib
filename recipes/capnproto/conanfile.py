from build import *


class Capnproto(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Cap'n Proto serialization/RPC system"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.3]",
    )
    requires = ("zlib/[^1.2.11]",)

    def source(self):
        self.get(f"https://github.com/capnproto/capnproto/archive/v{self.version}.tar.gz")

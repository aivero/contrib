from build import *


class Libnghttp2(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Framing layer of HTTP/2 is implemented as a reusable C library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )

    def source(self):
        self.get(
            f"https://github.com/nghttp2/nghttp2/releases/download/v{self.version}/nghttp2-{self.version}.tar.xz"
        )

from build import *


class Libwebp(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "library to encode and decode images in WebP format"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )

    def source(self):
        self.get(f"https://github.com/webmproject/libwebp/archive/v{self.version}.tar.gz")

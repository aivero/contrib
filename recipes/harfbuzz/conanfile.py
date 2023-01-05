from build import *


class Harfbuzz(Recipe):
    description = "HarfBuzz text shaping engine"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "gobject-introspection/[^1.66.1]",
    )
    requires = (
        ("freetype-no-harfbuzz/[^2.10.3]", "private"),
        "libpng/[^1.6.37]",
    )

    def source(self):
        self.get(f"https://github.com/harfbuzz/harfbuzz/archive/{self.version}.tar.gz")

    def build(self):
        opts = {
            "freetype": True,
            "gobject": True,
            "introspection": True,
        }
        self.meson(opts)

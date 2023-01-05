from build import *


class Graphene(Recipe):
    description = "A thin layer of graphic data types"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "gobject-introspection/[^1.66.1]",
    )
    requires = ("glib/[^2.70.3]",)

    def source(self):
        self.get(f"https://github.com/ebassi/graphene/archive/refs/tags/{self.version}.tar.gz")

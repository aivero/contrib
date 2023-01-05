from build import *


class PythonGobjectRecipe(PythonRecipe):
    description = "Python GObject bindings"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.51.2]",
    )
    requires = (
        "gobject-introspection/[^1.59.3]",
        "python-cairo/[^1.18.2]",
    )

    def source(self):
        self.get(
            f"https://gitlab.gnome.org/GNOME/pygobject/-/archive/{self.version}/pygobject-{self.version}.tar.gz"
        )

    def build(self):
        opts = {
            "pycairo": True,
        }
        self.meson(opts)

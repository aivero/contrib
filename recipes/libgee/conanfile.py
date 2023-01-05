from build import *


class Libgee(Recipe):
    description = "A collection library providing GObject-based interfaces and classes for commonly used data structures"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.61.1]",
        "gobject-introspection/[^1.69.0]",
    )
    requires = ("glib/[^2.70.3]",)

    def source(self):
        self.get(
            f"https://github.com/frida/libgee/archive/5b00fd64096b369a04fe04a7246e1927c3fedbd7.zip"
        )

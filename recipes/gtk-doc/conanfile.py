from build import *


class GtkDoc(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Generate API documentation from comments added to C code,typically used to document the public API of GTK+ and GNOME libraries."
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.0]",
        "itstool/[^2.0.6]",
        "gettext/[^0.21]",
    )
    requires = (
        "python-pygments/[^2.8.1]",
        "libxslt/[^1.1.34]",
        "glib/[^2.70.3]",
    )

    def source(self):
        self.get(f"https://github.com/GNOME/gtk-doc/archive/refs/tags/{self.version}.tar.gz")

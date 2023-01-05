from build import *


class Libxslt(Recipe):
    description = "XML stylesheet transformation library"
    license = "custom"
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")
    requires = ("libxml2/[^2.9.10]",)

    def source(self):
        self.get(
            f"https://gitlab.gnome.org/GNOME/libxslt/-/archive/v{self.version}/libxslt-v{self.version}.tar.gz"
        )

    def build(self):
        args = [
            "--without-python",
        ]
        self.autotools(args)

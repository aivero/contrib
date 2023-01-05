from build import *


class Dbus(Recipe):
    description = "Freedesktop.org message bus system"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "autoconf-archive/[>=2019.01.06]",
    )
    requires = ("expat/[^2.2.7]",)

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/dbus/dbus/-/archive/dbus-{self.version}/dbus-dbus-{self.version}.tar.bz2"
        )

from build import *


class AtSpi2Core(CppRecipe):
    description = "Protocol definitions and daemon for D-Bus at-spi"
    license = "LGPL"
    build_requires = ("cc/[^1.0.0]", "meson/[>=0.55.3]")
    requires = (
        "glib/[^2.70.3]",
        "dbus/[^1.12.16]",
    )

    def source(self):
        version = self.version.replace(".", "_")
        self.get(
            f"https://gitlab.gnome.org/GNOME/at-spi2-core/-/archive/AT_SPI2_CORE_{version}/at-spi2-core-AT_SPI2_CORE_{version}.tar.gz"
        )

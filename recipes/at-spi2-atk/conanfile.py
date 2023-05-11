from build import *


class AtSpi2Atk(CppRecipe):
    description = "A GTK+ module that bridges ATK to D-Bus at-spi"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
    )
    requires = (
        "atk/[^2.36.0]",
        "at-spi2-core/[^2.38.0]",
        "libxml2/[^2.9.9]",
    )

    def source(self):
        version = self.version.replace(".", "_")
        self.get(
            f"https://gitlab.gnome.org/GNOME/at-spi2-atk/-/archive/AT_SPI2_ATK_{version}/at-spi2-atk-AT_SPI2_ATK_{version}.tar.bz2"
        )

from build import *


class Libxkbcommon(Recipe):
    description = "Keymap handling library for toolkits and window systems"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "bison/[^3.3]",
        "flex/[^2.6.4]",
    )
    requires = (
        "libxcb/[^1.13.1]",
        "libxml2/[^2.9.10]",
    )

    def source(self):
        self.get(
            f"https://github.com/xkbcommon/libxkbcommon/archive/xkbcommon-{self.version}.tar.gz"
        )

    def build(self):
        opts = {
            "enable-wayland": False,
            "enable-docs": False,
        }
        self.meson(opts)

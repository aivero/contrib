from build import *


class WaylandProtocols(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Specifications of extended Wayland protocols"
    license = "MIT"
    build_requires = ("meson/[>=0.57.0]",)
    requires = ("wayland/[^1.19.0]",)

    def source(self):
        self.get(
            f"https://wayland.freedesktop.org/releases/wayland-protocols-{self.version}.tar.xz"
        )

    def build(self):
        opts = {
            "tests": False,
        }
        self.meson(opts)

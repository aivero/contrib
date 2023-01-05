from build import *


class Wayland(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "A computer display server protocol"
    license = "MIT"
    exports = "disable-tests.patch"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.2]",
    )
    requires = (
        "libffi/[^3.3]",
        "expat/[^2.2.7]",
        "libxml2/[^2.9.10]",
    )

    def source(self):
        self.get(f"https://wayland.freedesktop.org/releases/wayland-{self.version}.tar.xz")

    def build(self):
        opts = {
            "documentation": False,
            "tests": False,
        }
        self.meson(opts)

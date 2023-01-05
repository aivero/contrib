from build import *


class WlClipboard(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Command-line copy/paste utilities for Wayland"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.57.0]",
        "wayland-protocols/[^1.21]",
    )
    requires = ("wayland/[^1.19.0]",)

    def source(self):
        self.get(f"https://github.com/bugaevc/wl-clipboard/archive/v{self.version}.tar.gz")

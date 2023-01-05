from build import *


class Libdrm(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Direct Rendering Manager headers and kernel modules"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "libpciaccess/[>=0.16]",
    )

    def source(self):
        self.get(f"http://dri.freedesktop.org/libdrm/libdrm-{self.version}.tar.xz")

    def build(self):
        opts = {
            "amdgpu": True,
            "nouveau": True,
            "radeon": True,
        }
        self.meson(opts)

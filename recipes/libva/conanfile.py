from build import *


class Libva(Recipe):
    description = "Libva is an implementation for VA-API (Video Acceleration API)"
    license = "MIT"
    options = {"x11": [True, False], "wayland": [True, False]}
    default_options = ("x11=True", "wayland=False")
    build_requires = ("cc/[^1.0.0]", "meson/[>=0.55.3]")
    requires = (
        "libdrm/[^2.4.114]",
        "libxext/[^1.3.4]",
        "libxfixes/[^5.0.3]",
    )

    def source(self):
        self.get(f"https://github.com/intel/libva/archive/{self.version}.tar.gz")

    def build(self):
        opts = {}
        opts["with_x11"] = self.options.x11
        opts["with_wayland"] = self.options.wayland
        self.meson(opts)

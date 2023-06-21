from build import *


class Gxr(Recipe):
    description = "A glib wrapper for the OpenVR and the OpenXR APIs"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
    )
    requires = (
        "gulkan/[^0.16.0]",
        "gtk3/[^3.24.28]",
        "json-glib/[^1.6.2]",
        "wayland/[^1.20.0]",
        "openxr/[^1.0.27]",
    )

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/xrdesktop/gxr/-/archive/{self.version}/gxr-{self.version}.tar.gz"
        )

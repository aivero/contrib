from build import *


class Monado(CppRecipe):
    description = "An open source OpenXR runtime"
    license = "Boost"
    options = {
        "x11": [True, False],
    }
    default_options = ("x11=True",)
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "glslang/[^11.2.0]",
    )
    requires = (
        "eigen/[^3.3.9]",
        "vulkan-icd-loader/[^1.2.172]",
        "libglvnd/[^1.3.2]",
        "libxrandr/[^1.5.2]",
    )

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/monado/monado/-/archive/v{self.version}/monado-v{self.version}.tar.bz2"
        )

    def build(self):
        opts = {
            "install-active-runtime": False,
            "opengl": True,
            "xlib": self.options.x11,
            "xcb": self.options.x11,
        }
        self.meson(opts)

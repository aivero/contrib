from build import *


class Monado(CppRecipe):
    description = "An open source OpenXR runtime"
    license = "boost"
    options = {
        "x11": [True, False],
    }
    default_options = ("x11=True",)
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
        "glslang/[^11.2.0]",
        "mesa/[>=20.2.1]",
    )
    requires = (
        "eigen/[^3.3.9]",
        "vulkan-icd-loader/[^1.2.172]",
        "libglvnd/[^1.3.2]",
        "libxrandr/[^1.5.2]",
        "hidapi/[^0.13.1]",
    )

    def source(self):
        self.get(
            "https://gitlab.freedesktop.org/TheJackiMonster/monado/-/archive/main/monado-main.tar.gz"
        )

    def build(self):
        opts = {
            "install-active-runtime": False,
            "opengl": True,
            "xlib": self.options.x11,
            "xcb": self.options.x11,
        }
        #self.meson(opts)
        os.environ["CPATH"] += ":" + ":".join(
            self.deps_cpp_info["libx11"].include_paths
            + self.deps_cpp_info["xorgproto"].include_paths
            + self.deps_cpp_info["libxcb"].include_paths
            + self.deps_cpp_info["libxrandr"].include_paths
            + self.deps_cpp_info["libxrender"].include_paths
        )
        defs = {
            "XRT_HAVE_OPENGLES": False,
            "XRT_HAVE_EGL": False,
            "XRT_BUILD_DRIVER_NA": True,
        }
        self.cmake(defs)

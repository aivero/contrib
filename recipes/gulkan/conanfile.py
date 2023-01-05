from build import *


class Gulkan(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "A GLib library for Vulkan abstraction"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
    )
    requires = (
        "vulkan-icd-loader/[^1.2.172]",
        "glslang/[^11.2.0]",
        "gdk-pixbuf2/[^2.40.0]",
        "graphene/[^1.10.6]",
    )

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/xrdesktop/gulkan/-/archive/{self.version}/gulkan-{self.version}.tar.gz"
        )

    def build(self):
        os.environ[
            "CFLAGS"
        ] += f" -I{os.path.join(self.deps_cpp_info['vulkan-headers'].rootpath, 'include')}"

        opts = {
            "examples": False,
        }
        self.meson(opts)

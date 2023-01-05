from build import *


class VulkanIcdLoader(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Vulkan Installable Client Driver (ICD) Loader"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.1]",
        "xorgproto/[>=2020.1]",
        "libxrandr/[^1.5.2]",
        "libxcb/[^1.14]",
        "libxrandr/[^1.5.2]",
        "libxrender/[^0.9.10]",
        "wayland/[^1.19.0]",
        "git/[^2.30.0]",
    )
    requires = ("vulkan-headers/[^1.2.174]",)

    def source(self):
        self.get(f"https://github.com/KhronosGroup/Vulkan-Loader/archive/v{self.version}.tar.gz")

    def build(self):
        for req in ["libxcb", "xorgproto", "libx11", "libxrandr", "libxrender"]:
            os.environ[
                "CFLAGS"
            ] += f" -I{os.path.join(self.deps_cpp_info[req].rootpath, 'include')}"
        defs = {"VULKAN_HEADERS_INSTALL_DIR": self.deps_cpp_info["vulkan-headers"].rootpath}
        self.cmake(defs)

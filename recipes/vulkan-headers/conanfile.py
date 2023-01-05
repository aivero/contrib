from build import *


class VulkanHeaders(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Vulkan header files"
    license = "apache"
    build_requires = ("cc/[^1.0.0]", "cmake/[^3.18.0]")

    def source(self):
        self.get(f"https://github.com/KhronosGroup/Vulkan-Headers/archive/v{self.version}.tar.gz")

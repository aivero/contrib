from build import *


class SpirvTools(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "API and commands for processing SPIR-V modules"
    license = "custom"
    build_requires = (
        "cmake/[^3.18.4]",
        "python/[^3]",
        "git/[^2.29.1]",
        "ninja/[^1.10.0]",
    )

    def source(self):
        self.get(f"https://github.com/KhronosGroup/SPIRV-Headers/archive/{self.version}.tar.gz")

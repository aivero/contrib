from build import *


class SpirvTools(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "API and commands for processing SPIR-V modules"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
        "python/[^3]",
        "git/[^2.29.1]",
    )

    def source(self):
        self.get(
            "https://github.com/KhronosGroup/SPIRV-Headers/archive/master.tar.gz", "spirv-headers"
        )
        self.get(f"https://github.com/KhronosGroup/SPIRV-Tools/archive/v{self.version}.tar.gz")

    def build(self):
        defs = {
            "SPIRV-Headers_SOURCE_DIR": os.path.join("spirv-headers"),
        }
        self.cmake(defs)

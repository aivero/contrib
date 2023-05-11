from build import *


class SpirvTools(CppPythonRecipe):
    description = "API and commands for processing SPIR-V modules"
    license = "custom"
    build_requires = (
        "cmake/[^3.18.4]",
        "git/[^2.29.1]",
        "ninja/[^1.10.0]",
    )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/KhronosGroup/SPIRV-Headers/archive/{self.version}.tar.gz")

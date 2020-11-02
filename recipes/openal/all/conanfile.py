from build import *


class OpenalRecipe(Recipe):
    description = "Cross-platform 3D audio library, software implementation"
    license = "custom"
    build_requires = ("cc/[^1.0.0]", "cmake/[^3.18.4]")
    requires = ("libffi/3.3",)

    def source(self):
        self.get(f"https://github.com/kcat/openal-soft/archive/openal-soft-{self.version}.tar.gz")

from build import *


class Glslang(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "OpenGL and OpenGL ES shader front end and validator"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.1]",
    )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/KhronosGroup/glslang/archive/{self.version}.tar.gz")

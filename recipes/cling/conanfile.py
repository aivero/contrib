from build import *


class Cling(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "Interactive C++ interpreter, built on the top of LLVM and Clang libraries"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "git/[^2.30.0]",
        "cmake/[^3.18.4]",
    )
    requires = ("libxml2/[^2.9.10]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/root-project/cling/archive/refs/tags/v{self.version}.tar.gz")

    def build(self):
        defs = {
            "LLVM_BUILD_LLVM_DYLIB": True,
        }
        self.cmake()

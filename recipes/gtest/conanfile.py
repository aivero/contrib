from build import *


class GTest(CppRecipe):
    description = "Google's C++ test framework"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )

    def source(self):
        self.get(f"https://github.com/google/googletest/archive/release-{self.version}.tar.gz")

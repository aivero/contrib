from build import *


class nlopt(Recipe):
    description = "library for nonlinear optimization, wrapping many algorithms for global and local, constrained or unconstrained, optimization "
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )

    def source(self):
        self.get(f"https://github.com/stevengj/nlopt/archive/refs/tags/v{self.version}.tar.gz")

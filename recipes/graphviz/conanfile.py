from build import *


class Graphviz(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Graph Visualization Tools"
    license = "EPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "flex/[^2.6.4]",
        "bison/[^3.3]",
    )

    def source(self):
        self.get(
            f"https://www2.graphviz.org/Packages/stable/portable_source/graphviz-{self.version}.tar.gz"
        )

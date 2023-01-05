from build import *


class NodeGyp(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Node.js native addon build tool"
    license = "MIT"
    requires = ("nodejs/[>=16.6.1]",)

    def source(self):
        tools.download(
            f"https://registry.npmjs.org/node-gyp/-/node-gyp-{self.version}.tgz",
            f"{self.name}-{self.version}.tgz",
        )

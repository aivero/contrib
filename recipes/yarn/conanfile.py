from build import *


class Yarn(Recipe):
    description = "Fast, reliable, and secure dependency management"
    license = "BSD"
    build_requires = ("npm/[>=7.0.5]",)
    requires = ("nodejs/[>=16.6.1]",)

    def source(self):
        self.get(
            f"https://github.com/yarnpkg/yarn/releases/download/v{self.version}/yarn-v{self.version}.tar.gz"
        )

from build import *


class Flex(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Flex, the fast lexical analyzer generator"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "bison/[^3.7.2]",
    )

    def source(self):
        self.get(
            f"https://github.com/westes/flex/releases/download/v{self.version}/flex-{self.version}.tar.gz"
        )

    def build(self):
        # args = [
        #     "--disable-nls",
        #     "ac_cv_func_reallocarray=no",
        # ]
        # self.autotools(args)
        self.autotools()

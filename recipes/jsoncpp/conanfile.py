from build import *


class Jsoncpp(Recipe):
    description = "C++ library for interacting with JSON"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
    )

    def source(self):
        self.get(f"https://github.com/open-source-parsers/jsoncpp/archive/{self.version}/jsoncpp-{self.version}.tar.gz")

    def build(self):
        self.meson()

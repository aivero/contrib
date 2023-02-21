from build import *


class Libyaml(Recipe):
    description = "YAML 1.1 library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://pyyaml.org/download/libyaml/yaml-{self.version}.tar.gz")

    def build(self):
        self.autotools()

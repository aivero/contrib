from build import *


class Libepoxy(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Library handling OpenGL function pointer management"
    license = "MIT"
    options = {
        "x11": [True, False],
    }
    default_options = ("x11=True",)
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
    )
    requires = ("mesa/[>=20.2.1]",)

    def source(self):
        self.get(f"https://github.com/anholt/libepoxy/archive/{self.version}.tar.gz")

    def build(self):
        opts = {
            "glx": True,
            "tests": False,
            "x11": self.options.x11,
        }
        self.meson(opts)

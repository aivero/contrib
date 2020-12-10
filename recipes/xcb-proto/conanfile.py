from build import *


class XcbProtoRecipe(Recipe):
    description = "XML-XCB protocol descriptions"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://xcb.freedesktop.org/dist/xcb-proto-{self.version}.tar.xz")

from build import *


class IntelMediaSdk(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = (
        "API to access hardware-accelerated video on Intel Gen graphics hardware platforms"
    )
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.8.4]",
    )
    requires = ("libva/[^2.13.0]",)
    exports_sources = "lld-linking.patch"

    def source(self):
        self.get(
            f"https://github.com/Intel-Media-SDK/MediaSDK/archive/intel-mediasdk-{self.version}.tar.gz"
        )
        self.patch("lld-linking.patch")

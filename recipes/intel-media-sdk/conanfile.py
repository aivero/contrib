from build import *


class IntelMediaSdkRecipe(Recipe):
    description = "API to access hardware-accelerated video on Intel Gen graphics hardware platforms"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.8.4]",
    )

    def source(self):
        self.get(f"https://github.com/Intel-Media-SDK/MediaSDK/archive/intel-mediasdk-{self.version}.tar.gz")

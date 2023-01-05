from build import *


class OpenCV(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "OpenCV is an open source computer vision and machine learning software library."
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.3]",
        "zlib/[^1.2.11]",
        "libpng/[^1.6.37]",
    )

    def source(self):
        self.get(f"https://github.com/opencv/opencv/archive/{self.version}.tar.gz")

    def build(self):
        defs = {
            "OPENCV_GENERATE_PKGCONFIG": True,
            "BUILD_ZLIB": True,
            "BUILD_JPEG_TURBO_DISABLE": True,
            "BUILD_PNG": True,
            "BUILD_TIFF": False,
            "BUILD_JASPER": False,
            "BUILD_JPEG": False,
            "BUILD_OPENEXR": False,
            "BUILD_WEBP": False,
            "BUILD_TBB": False,
            "BUILD_IPP_IW": False,
            "BUILD_ITT": False,
        }
        self.cmake(defs)

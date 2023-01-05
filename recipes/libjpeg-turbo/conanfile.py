from build import *


class LibjpegTurbo(Recipe):
    description = "JPEG image codec with accelerated baseline compression and decompression"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
        "yasm/[^1.3.0]",
        "zlib/[^1.2.11]",
    )

    def source(self):
        self.get(
            f"https://downloads.sourceforge.net/project/libjpeg-turbo/{self.version}/libjpeg-turbo-{self.version}.tar.gz"
        )

    def build(self):
        defs = {
            "WITH_JPEG8": True,
            "CMAKE_INSTALL_LIBDIR": os.path.join(self.package_folder, "lib"),
            "CMAKE_INSTALL_INCLUDEDIR": os.path.join(self.package_folder, "include"),
            "ENABLE_STATIC": False,
        }
        self.cmake(defs)

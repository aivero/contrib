from build import *


class Mozjpeg(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "JPEG image codec with accelerated baseline compression and decompression"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
        "yasm/[^1.3.0]",
        "zlib/[^1.2.11]",
        "libpng/[^1.6.37]",
    )

    def source(self):
        self.get(f"https://github.com/mozilla/mozjpeg/archive/v{self.version}.tar.gz")

    def build(self):
        defs = {
            "CMAKE_INSTALL_BINDIR": os.path.join(self.package_folder, "bin"),
            "CMAKE_INSTALL_DATAROOTDIR": os.path.join(self.package_folder, "share"),
            "CMAKE_INSTALL_INCLUDEDIR": os.path.join(self.package_folder, "include"),
            "CMAKE_INSTALL_LIBDIR": os.path.join(self.package_folder, "lib"),
        }
        if self.options.shared:
            defs["ENABLE_STATIC"] = False
        else:
            defs["ENABLE_SHARED"] = False
        self.cmake(defs)

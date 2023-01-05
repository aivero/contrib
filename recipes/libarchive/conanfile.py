from build import *


class Libarchive(Recipe):
    description = "Multi-format archive and compression library"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )
    requires = (
        "zlib/[^1.2.11]",
        "bzip2/[^1.0.8]",
        "xz/[^5.2.5]",
        "expat/[^2.2.7]",
        "openssl1/[>=1.1.1h]",
    )

    def source(self):
        self.get(
            f"https://github.com/libarchive/libarchive/releases/download/v{self.version}/libarchive-{self.version}.tar.xz"
        )

    def build(self):
        self.cmake()
        if self.options.shared:
            os.remove(os.path.join(self.package_folder, "lib", "libarchive.a"))
        else:
            os.remove(os.path.join(self.package_folder, "lib", "libarchive.so"))

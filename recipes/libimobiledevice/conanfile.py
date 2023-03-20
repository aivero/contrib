from build import *


class Libimobiledevice(PythonRecipe):
    description = "Library that talks the protocols to support iPhone and iPod Touch devices on Linux"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "cython/[^0.29.33]",
        "autoconf-archive/[^2021.02.19]",
    )
    requires = (
        "gnutls/[^3.7.2]",
        "libgcrypt/[^1.10.1]",
        "libusbmuxd/[^2.0.2]",
    )

    def source(self):
        self.get(f"https://github.com/libimobiledevice/libimobiledevice/archive/refs/tags/{self.version}.tar.gz")
    
    def build(self):
        os.environ["CFLAGS"] += "-Wno-int-conversion"
        self.autotools()

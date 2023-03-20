from build import *


class Libusbmuxd(Recipe):
    description = "USB Multiplex Daemon"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = (
        "libusb/[^1.0.23]",
        "libplist/[^2.2.0]",
    )

    def source(self):
        self.get(f"https://github.com/libimobiledevice/libusbmuxd/archive/refs/tags/{self.version}.tar.gz")
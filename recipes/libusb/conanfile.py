from build import *


class LibUSB(Recipe):
    description = "A cross-platform library to access USB devices"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = ("eudev/[^3.2.9]",)

    def source(self):
        self.get(
            f"https://github.com/libusb/libusb/releases/download/v{self.version}/libusb-{self.version}.tar.bz2"
        )

    def build(self):
        args = [
            "--enable-udev",
        ]
        self.autotools(args)

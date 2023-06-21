from build import *


class Hidapi(Recipe):
    description = "Simple library for communicating with USB and Bluetooth HID devices"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )
    requires = (
        "libusb/[^1.0.23]",
    )

    def source(self):
        self.get(f"https://github.com/libusb/hidapi/archive/hidapi-{self.version}.tar.gz")

    def build(self):
        self.cmake()
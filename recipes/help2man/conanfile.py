from build import *


class Help2Man(Recipe):
    description = "Conversion tool to create man files"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/help2man/help2man-{self.version}.tar.xz")

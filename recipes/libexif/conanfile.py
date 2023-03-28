from build import *


class LibExif(Recipe):
    description = "A library for parsing, editing, and saving EXIF data"
    license = "LGPL"
    build_requires = (
        "autotools/[^1.0.0]"
    )

    def source(self):
        self.get(f"https://github.com/libexif/libexif/archive/refs/tags/v{self.version}.tar.gz")
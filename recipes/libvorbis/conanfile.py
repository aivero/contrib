from build import *


class Libvorbis(Recipe):
    description = "Reference implementation of the Ogg Vorbis audio format"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = (
        "libogg/[^1.3.5]",
    )

    def source(self):
        self.get(f"https://github.com/xiph/vorbis/archive/refs/tags/v{self.version}.tar.gz")
from build import *


class Libcanberra(Recipe):
    description = "A small and lightweight implementation of the XDG Sound Theme Specification"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "gtk3/[^3.24.31]",
    )
    requires = (
        "libvorbis/[^1.3.7]",
        #"libltdl/[^]",
        #"alsa-lib/[^1.2.9]",
        #"libpulse/[^]",
        #"tdb/[^1.4.8]",
        #"sound-theme-freedesktop/[^0.8]",
    )

    def source(self):
        self.get(f"http://0pointer.de/lennart/projects/libcanberra/libcanberra-{self.version}.tar.xz")

    def build(self):
        args = []
        self.autotools(args)

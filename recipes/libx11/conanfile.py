from build import *


class Libx11(Recipe):
    description = "X11 client-side library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "pkgconf/[^1.7.3]",
        "perl/[^5.30.0]",
        "xorg-util-macros/[^1.19.2]",
        "xorgproto/[>=2020.1]",
    )
    requires = (
        "xtrans/[^1.4.0]",
        "libxcb/[^1.14]",
    )

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/lib/libX11-{self.version}.tar.gz"
        )

    def package_info(self):
        self.env_info.XLOCALEDIR = os.path.join(self.package_folder, "share", "X11", "locale")

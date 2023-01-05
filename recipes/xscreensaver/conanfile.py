from build import *


class Xscreensaver(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Screen saver and locker for the X Window System"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        # "bc/[^1.07.1]",
        # "intltool/[^0.51.0]",
        # "libxpm/[^3.5.13]",
        # "gdm/[^41.3]",
    )
    requires = (
        # "glu/[^9.0.2]",
        # "xorg-appres/[^1.0.5]",
        "libglvnd/[^1.3.2]",
        # "libxcrypt/[^4.4.28]",
        # "libxft/[^2.3.4]",
        "libxi/[^1.7.10]",
        "libxinerama/[^1.1.4]",
        # "libxmu/[^1.1.3]",
        "libxrandr/[^1.5.2]",
        # "libxt/[^1.2.1]",
        "libxxf86vm/[^1.1.4]",
        # "pam/[^1.5.2]",
        "gdk-pixbuf2/[^2.42.6]",
        # "gdk-pixbuf-xlib/[^2.40.2]",
    )

    def source(self):
        self.get(f"https://www.jwz.org/xscreensaver/xscreensaver-{self.version}.tar.gz")

    def build(self):
        # TODO: port to Meson
        args = []
        self.autotools(args)

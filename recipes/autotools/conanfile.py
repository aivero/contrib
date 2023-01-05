from build import *


class Autotools(Recipe):
    description = "A suite of programming tools 'designed' to assist in making source code"
    license = "GPL"
    requires = (
        "automake/[^1.16.1]",
        "libtool/[^2.4.6]",
        "pkgconf/[^1.7.3]",
        "gettext/[^0.21]",
        "texinfo/[^6.6]",
    )

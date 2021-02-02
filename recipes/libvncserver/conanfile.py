from build import *


class LibvncserverRecipe(Recipe):
    description = "Cross-platform C libraries that allow you to easily implement VNC server or client functionality"
    license = "Apache"
    build_requires = ("cc/[^1.0.0]", "cmake/[^3.18.4]")
    requires = (
        "libpng/[^1.6.37]",
        "openssl/[^1.1.1b]",
    )

    def source(self):
        self.get(f"https://github.com/LibVNC/libvncserver/archive/LibVNCServer-{self.version}.tar.gz")
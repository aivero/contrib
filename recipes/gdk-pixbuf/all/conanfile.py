import os
from conans import *


class GdkPixbufConan(ConanFile):
    description = "An image loading library"
    license = "LGPL-2.1"
    settings = "build_type", "compiler", "arch_build", "os_build", "libc_build"
    build_requires = (
        "meson/[^0.55.3]",
        "gobject-introspection/[^1.66.1]",
        "gettext/[^0.21]",
    )
    requires = (
        "libx11/[^1.6.8]",
        "libpng/[^1.6.37]",
        "shared-mime-info/[^2.0]",
    )

    def source(self):
        tools.get(f"https://github.com/GNOME/gdk-pixbuf/archive/{self.version}.tar.gz")
        # Disable broken tests
        tools.replace_in_file(os.path.join(f"gdk-pixbuf-{self.version}", "meson.build"), "subdir('tests')", "")

    def build(self):
        args = [
            "--auto-features=disabled",
            "--wrap-mode=nofallback",
            "-Dinstalled_tests=false",
            "-Drelocatable=true",
        ]
        meson = Meson(self)
        meson.configure(args, source_folder=f"gdk-pixbuf-{self.version}", pkg_config_paths=os.environ["PKG_CONFIG_PATH"].split(":"))
        meson.install()

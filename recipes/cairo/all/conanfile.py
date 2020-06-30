import os

from conans import *


class CairoConan(ConanFile):
    description = "2D graphics library with support for multiple output devices"
    license = "LGPL"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    options = {
        "introspection": [True, False],
        "zlib": [True, False],
        "png": [True, False],
        "fontconfig": [True, False],
    }
    default_options = ("introspection=True", "zlib=True", "png=True", "fontconfig=True")
    scm = {
        "type": "git",
        "url": "https://github.com/centricular/cairo.git",
        "revision": "meson-%s" % version,
        "recursive": True,
        "subfolder": ("cairo-%s" % version),
    }

    def build_requirements(self):
        self.build_requires("generators/1.0.0")
        self.build_requires("meson/[^0.51.2]")
        if self.options.introspection:
            self.build_requires("gobject-introspection/[^1.59.3]")

    def requirements(self):
        self.requires("glib/[^2.62.0]")
        self.requires("pixman/[^0.38.4]")
        self.requires("libxrender/[^0.9.10]")
        self.requires("libxext/[^1.3.4]")
        if self.options.fontconfig:
            self.requires("fontconfig/[^2.13.1]")
        if self.options.zlib:
            self.requires("zlib/[^1.2.11]")
        if self.options.png:
            self.requires("libpng/[^1.6.37]")

    def build(self):
        meson = Meson(self)
        args = ["-Dintrospection=" + ("enabled" if self.options.introspection else "disabled")]
        args.append("-Dfontconfig=" + ("enabled" if self.options.fontconfig else "disabled"))
        args.append("-Dzlib=" + ("enabled" if self.options.zlib else "disabled"))
        args.append("-Dpng=" + ("enabled" if self.options.png else "disabled"))

        meson.configure(
            source_folder="cairo-" + self.version, args=args, pkg_config_paths=os.environ["PKG_CONFIG_PATH"].split(":"),
        )
        meson.install()

    def package_info(self):
        self.env_info.GI_TYPELIB_PATH.append(os.path.join(self.package_folder, "lib", "girepository-1.0"))

from build import *


class GdkPixbuf2(Recipe):
    librsvg_version = "2.50.4"
    description = "An image loading library"
    license = "LGPL"
    options = {
        "shared": [True, False],
        "introspection": [True, False],
    }
    default_options = (
        "shared=True",
        "introspection=True",
    )
    build_requires = (
        "cc/[^1.0.0]",
        "rust/[^1.0.0]",
        "meson/[>=0.55.3]",
        "autotools/[^1.0.0]",
        "gtk-doc/[^1.33.2]",
        "gobject-introspection/[^1.66.1]",
        "gettext/[^0.21]",
    )
    requires = (
        "shared-mime-info/[^2.0]",
        "pango/[^1.48.4]",
        "libtiff/[^4.3.0rc1]",
    )

    def source(self):
        self.get(f"https://github.com/GNOME/gdk-pixbuf/archive/{self.version}.tar.gz")
        # Disable broken tests
        tools.replace_in_file(os.path.join(self.src, "meson.build"), "subdir('tests')", "")
        self.get(f"https://github.com/GNOME/librsvg/archive/refs/tags/{self.librsvg_version}.tar.gz", f"librsvg-{self.librsvg_version}.src")

    def build(self):
        opts = {
            "installed_tests": False,
            "relocatable": True,
            "introspection": self.options.introspection,
            "man": False,
        }
        self.meson(opts)
        if self.name == "gdk-pixbuf2":
            os.environ["PKG_CONFIG_PATH"] += f":{os.path.join(self.package_folder, 'lib', 'pkgconfig')}"
            os.environ["PATH"] += f":{os.path.join(self.package_folder, 'bin')}"
            os.environ["XDG_DATA_DIRS"] += f":{os.path.join(self.package_folder, 'share')}"
            self.autotools(source_folder=f"librsvg-{self.librsvg_version}.src")

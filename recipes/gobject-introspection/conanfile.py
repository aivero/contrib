from build import *


class GObjectIntrospection(PythonRecipe):
    settings = PythonRecipe.settings + ("compiler",)
    description = "Middleware layer between C libraries (using GObject) and language bindings"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "bison/[^3.7.2]",
        "flex/[^2.6.4]",
    )
    requires = ("glib/[^2.70.3]",)

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        # Workaround: https://gitlab.gnome.org/GNOME/gobject-introspection/-/issues/414
        # Upgrade to 1.71.0 when available
        version = "effb1e09dee263cdac4ec593e8caf316e6f01fe2"
        self.get(f"https://github.com/GNOME/gobject-introspection/archive/{version}.tar.gz")

    def package_info(self):
        self.env_info.PYTHONPATH.append(
            os.path.join(self.package_folder, "lib", "gobject-introspection")
        )

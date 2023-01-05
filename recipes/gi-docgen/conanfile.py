from build import *


class GiDocgen(PythonRecipe):
    description = "Documentation generator for GObject-based libraries"
    license = "Apache"
    build_requires = (
        "python-setuptools/[>=41.2.0]",
        "meson/[>=0.55.3]",
    )
    requires = (
        "python-markdown/[^3.3.6]",
        "python-typogrify/[^2.0.7]",
        "python-smartypants/[^2.0.1]",
        "python-jinja/[^3.0.0]",
        "python-toml/[>=0.10.2]",
        "python-pygments/[^2.11.2]",
    )

    def source(self):
        self.get(
            f"https://gitlab.gnome.org/GNOME/gi-docgen/-/archive/{self.version}/gi-docgen-{self.version}.tar.gz"
        )

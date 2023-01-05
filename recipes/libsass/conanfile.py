from build import *


class Libsass(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "C implementation of Sass CSS preprocessor (library)."
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://github.com/sass/libsass/archive/{self.version}.tar.gz")

    def build(self):
        os.environ["LIBSASS_VERSION"] = self.version
        # TODO: libsass does not work with autotools()
        self.exe("autoreconf -i")
        self.exe(f"sh configure --prefix={self.package_folder}")
        self.exe("make")
        self.exe("make install")

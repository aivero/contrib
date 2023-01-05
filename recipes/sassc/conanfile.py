from build import *


class Sassc(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "C implementation of Sass CSS preprocessor."
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = ("libsass/[^3.6.5]",)

    def source(self):
        self.get(f"https://github.com/sass/sassc/archive/{self.version}.tar.gz")

    def build(self):
        # TODO: sassc does not work with autotools()
        self.exe("autoreconf -i")
        self.exe(
            f"sh configure --with-libsass={self.deps_cpp_info['libsass'].rootpath} --prefix={self.package_folder}"
        )
        self.exe("make")
        self.exe("make install")

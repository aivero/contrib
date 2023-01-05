from build import *


class Libimagequant(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = (
        "Library for high-quality conversion of RGBA images to 8-bit indexed-color (palette) images"
    )
    license = "BSD"
    build_requires = (
        "autotools/1.0.0",
        "openmp/[>=11.0.0]",
    )

    def source(self):
        self.get(
            f"https://github.com/ImageOptim/libimagequant/archive/{self.version}/libimagequant-{self.version}.tar.gz"
        )

    def build(self):
        os.environ["DESTDIR"] = self.package_folder
        args = [
            f"--prefix={self.package_folder}",
            "--with-openmp",
        ]
        self.autotools(args)

        os.remove(os.path.join(self.package_folder, "lib", "libimagequant.a"))

from build import *


class Imagemagick(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "An image viewing/manipulation program"
    license = "ImageMagick"
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")
    requires = ("libpng/[^1.6.37]",)

    def source(self):
        self.tar_version = "-".join(
            [self.version[: self.version.rfind(".")], self.version[self.version.rfind(".") + 1 :]]
        )
        self.get(f"https://github.com/ImageMagick/ImageMagick/archive/{self.tar_version}.tar.gz")

    def package_info(self):
        self.env_info.MAGICK_CONFIGURE_PATH = os.path.join(
            self.package_folder, "etc", "ImageMagick-" + self.version.split(".")[0]
        )

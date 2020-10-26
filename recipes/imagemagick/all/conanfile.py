from build import *


class ImagemagickRecipe(Recipe):
    description = "An image viewing/manipulation program"
    license = "GPL2"
    build_requires = ("autotools/[^1.0.0]",)
    requires = ("libpng/[^1.6.37]",)

    def source(self):
        self.tar_version = "-".join([self.version[: self.version.rfind(".")], self.version[self.version.rfind(".") + 1 :]])
        self.get(f"https://github.com/ImageMagick/ImageMagick/archive/{self.tar_version}.tar.gz")

    def build(self):
        args = [
            "--disable-static",
            "--disable-dependency-tracking",
        ]
        self.autotools(args)

    def package_info(self):
        self.env_info.MAGICK_CONFIGURE_PATH = os.path.join(self.package_folder, "etc", "ImageMagick-" + self.version.split(".")[0])

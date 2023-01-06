from build import *


class IntelMediaDriver(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Intel Media Driver for VAAPI Broadwell iGPUs"
    license = "MIT"
    build_requires = ("cc/[^1.0.0]", "cmake/[^3.18.4]")
    requires = (
        "intel-gmmlib/[^22.1.3]",
        "libva/[^2.13.0]",
        "libpciaccess/[>=0.16]",
    )

    def source(self):
        self.get(f"https://github.com/intel/media-driver/archive/intel-media-{self.version}.tar.gz")

    def package(self):
        self.copy("iHD_drv_video.so", src="media_driver", dst="lib")

    def build(self):
        os.environ["CPATH"] += ":" + ":".join(
            self.deps_cpp_info["libx11"].include_paths
            + self.deps_cpp_info["xorgproto"].include_paths
        )
        defs = {"BYPASS_MEDIA_ULT": "yes"}
        self.cmake(defs)

    def package_info(self):
        self.env_info.LIBVA_DRIVERS_PATH = os.path.join(self.package_folder, "lib")
        self.env_info.LIBVA_DRIVER_NAME = "iHD"

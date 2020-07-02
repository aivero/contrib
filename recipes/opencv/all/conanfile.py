import os

from conans import *


class OpenCVConan(ConanFile):
    description = "OpenCV is an open source computer vision and machine learning software library."
    license = "BSD"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    build_requires = ("cmake/[^3.15.3]",)
    requires = (
        "generators/[^1.0.0]",
        "zlib/[^1.2.11]",
        "libpng/[^1.6.37]",
    )

    def source(self):
        tools.get(f"https://github.com/opencv/opencv/archive/{self.version}.tar.gz")

    def build(self):
        cmake = CMake(self, generator="Ninja")
        cmake.definitions["OPENCV_GENERATE_PKGCONFIG"] = True
        cmake.definitions["BUILD_ZLIB"] = True
        cmake.definitions["BUILD_PNG"] = True
        cmake.definitions["BUILD_TIFF"] = False
        cmake.definitions["BUILD_JASPER"] = False
        cmake.definitions["BUILD_JPEG"] = False
        cmake.definitions["BUILD_OPENEXR"] = False
        cmake.definitions["BUILD_WEBP"] = False
        cmake.definitions["BUILD_TBB"] = False
        cmake.definitions["BUILD_IPP_IW"] = False
        cmake.definitions["BUILD_ITT"] = False
        cmake.definitions["BUILD_JPEG_TURBO_DISABLE"] = True
        cmake.configure(source_folder=f"{self.name}-{self.version}")
        cmake.build()
        cmake.install()

    def package_info(self):
        self.env_info.PYTHONPATH = os.path.join(self.package_folder, "lib", "python3.6", "dist-packages")

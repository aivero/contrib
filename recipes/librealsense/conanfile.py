from build import *


class LibRealsenseRecipe(PythonRecipe):
    description = "Intel RealSense SDK"
    license = "Apache"
    settings = PythonRecipe.settings + ("hardware",)
    exports = (
        "libusb-fix.patch",
        "pkgconfig-fix.patch",
        "cuda-clang-support.patch",
        "0001-Add-udev-include-dir.patch",
    )
    options = {"cuda": [True, False], "python": [True, False], "libuvc": [True, False]}
    default_options = ("cuda=False", "python=False", "libuvc=False")
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
        "git/[^2.30.0]",
    )
    requires = ("libusb/[^1.0.23]",)

    def requirements(self):
        if self.options.python:
            self.requires(f"python/[~{self.settings.python}]")

    def configure(self):
        if self.settings.arch == "armv8":
            self.options.libuvc = True
        if self.settings.hardware == "l4t":
            self.options.cuda = True

    def source(self):
        self.get(f"https://github.com/IntelRealSense/librealsense/archive/v{self.version}.tar.gz")
        self.patch("pkgconfig-fix.patch")
        self.patch("libusb-fix.patch")
        self.patch("cuda-clang-support.patch")
        self.patch("0001-Add-udev-include-dir.patch")

    def build(self):
        defs = {
            "BUILD_WITH_CUDA": self.options.cuda,
            "BUILD_PYTHON_BINDINGS": self.options.python,
            "BUILD_EXAMPLES": False,
            "BUILD_GRAPHICAL_EXAMPLES": False,
            "BUILD_PCL_EXAMPLES": False,
            "BUILD_NODEJS_BINDINGS": False,
            "BUILD_UNIT_TESTS": False,
            "CMAKE_CUDA_COMPILER": "clang++",
        }
        if self.options.libuvc:
            # Workaround for https://github.com/IntelRealSense/librealsense/issues/6656
            defs["FORCE_LIBUVC"] = True
        self.cmake(defs)

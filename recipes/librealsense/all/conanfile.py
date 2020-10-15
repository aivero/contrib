import os

from conans import *


class LibRealsenseConan(ConanFile):
    description = "Intel RealSense SDK"
    license = "Apache"
    settings = "build_type", "compiler", "arch_build", "os_build", "libc_build"
    exports = "libusb-fix.patch", "pkgconfig-fix.patch"
    build_requires = (
        "clang/[^10.0.1]",
        "cmake/[^3.18.4]",
        # "cuda/[^10.1.243]",
        "libusb/[^1.0.23]",
        "python/[^3.8.5]",
    )

    def source(self):
        tools.get(f"https://github.com/IntelRealSense/librealsense/archive/v{self.version}.tar.gz")
        tools.patch(patch_file="pkgconfig-fix.patch", base_path=f"{self.name}-{self.version}")
        tools.patch(patch_file="libusb-fix.patch", base_path=f"{self.name}-{self.version}")

    def build(self):
        env = {
            # "CMAKE_PROGRAM_PATH": self.deps_cpp_info["python"].rootpath,
            # "PythonInterp_ROOT": self.deps_cpp_info["python"].rootpath,
            "PATH": os.environ["PATH"].replace("/usr/bin:", "")
        }
        # with tools.environment_append(env):
        cmake = CMake(self)
        # cmake.definitions["BUILD_WITH_CUDA"] = self.options.cuda
        cmake.definitions["BUILD_PYTHON_BINDINGS"] = True
        cmake.definitions["BUILD_EXAMPLES"] = False
        cmake.definitions["BUILD_GRAPHICAL_EXAMPLES"] = False
        cmake.definitions["BUILD_PCL_EXAMPLES"] = False
        cmake.definitions["BUILD_NODEJS_BINDINGS"] = False
        cmake.definitions["BUILD_UNIT_TESTS"] = False
        cmake.definitions["PYTHON_INCLUDE_DIRS"] = os.path.join(self.deps_cpp_info["python"].rootpath, "include", "python3.8")
        cmake.definitions["PYTHON_LIBRARIES"] = os.path.join(self.deps_cpp_info["python"].rootpath, "lib", "libpython.a")
        cmake.definitions["PYTHONLIBS_FOUND"] = True
        cmake.definitions["PYTHON_MODULE_EXTENSION"] = "a"

        cmake.configure(source_folder=f"{self.name}-{self.version}")
        cmake.build()
        cmake.install()

    def package_info(self):
        self.env_info.PYTHONPATH.append(os.path.join(self.package_folder, "lib"))

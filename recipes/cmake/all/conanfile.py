from conans import *


class CMakeConan(ConanFile):
    description = "A cross-platform open-source make system"
    license = "custom"
    settings = "build_type", "compiler", "arch_build", "os_build", "libc_build"
    build_requires = (
        "bootstrap-cmake/[^3.18.0]",
        "bootstrap-ninja/[^1.10.0]",
        "clang/[^10.0.1]",
        "pkgconf/[^1.7.3]",
    )

    def source(self):
        tools.get(f"https://github.com/Kitware/CMake/releases/download/v{self.version}/cmake-{self.version}.tar.gz")

    def build(self):
        env = {"CXXFLAGS": os.environ["CXXFLAGS"] + ""}
        with tools.environment_append(env):
            cmake = CMake(self)
            cmake.definitions["CMAKE_USE_OPENSSL"] = False
            cmake.configure(source_folder=f"cmake-{self.version}")
            cmake.build()
            cmake.install()


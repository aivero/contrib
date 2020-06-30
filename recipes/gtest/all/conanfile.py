from conans import *


class GTestConan(ConanFile):
    description = "Google's C++ test framework"
    license = "BSD-3-Clause"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}

    def build_requirements(self):
        self.build_requires("generators/1.0.0")
        self.build_requires("cmake/[^3.15.3]")

    def source(self):
        tools.get("https://github.com/google/googletest/archive/release-%s.tar.gz" % self.version)

    def build(self):
        cmake = CMake(self, generator="Ninja")
        cmake.definitions["BUILD_SHARED_LIBS"] = "ON"
        cmake.configure(source_folder="googletest-release-" + self.version)
        cmake.install()

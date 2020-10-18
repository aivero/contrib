import os

from conans import *


class RapidJsonConan(ConanFile):
    description = "A fast JSON parser/generator for C++ with both SAX/DOM style API"
    license = "MIT"
    settings = "build_type", "compiler", "arch_build", "os_build", "libc_build"
    build_requires = ("cmake/[^3.15.3]",)

    def source(self):
        tools.get(f"https://github.com/Tencent/rapidjson/archive/{self.version}.tar.gz")

    def build(self):
        cmake = CMake(self, generator="Ninja")
        cmake.definitions["RAPIDJSON_HAS_STDSTRING"] = True
        cmake.definitions["RAPIDJSON_BUILD_CXX11"] = True
        cmake.definitions["RAPIDJSON_ENABLE_INSTRUMENTATION_OPT"] = True
        cmake.configure(source_folder=f"{self.name}-{self.version}")
        cmake.build()
        cmake.install()

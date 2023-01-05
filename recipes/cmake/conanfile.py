from build import *


class CMake(Recipe):
    description = "A cross-platform open-source make system"
    license = "custom"
    options = {}
    default_options = ()

    def build_requirements(self):
        if self.name == "cmake":
            self.build_requires(f"cmake-bootstrap/{self.version}")

    def requirements(self):
        if self.name == "cmake":
            self.requires("cc/[^1.0.0]")
            self.requires("ninja/[^1.10.0]")
            self.requires("pkgconf/[^1.7.3]")
            self.requires("openssl1/[>=1.1.1h]")

    def source(self):
        self.get(
            f"https://github.com/Kitware/CMake/releases/download/v{self.version}/cmake-{self.version}.tar.gz"
        )

    def build(self):
        defs = {"CMAKE_USE_OPENSSL": self.name == "cmake"}
        self.cmake(defs)

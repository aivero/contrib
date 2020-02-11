import os

from conans import CMake, ConanFile, tools


class OpenalConan(ConanFile):
    name = "openal"
    version = tools.get_env("GIT_TAG", "1.20.1")
    license = "custom", "Apache"
    description = "Cross-platform 3D audio library, software implementation"
    url = "https://gitlab.com/aivero/public/conan/conan-" + name
    settings = "os", "compiler", "arch", "build_type"
    generators ="pkgconf"

    def build_requirements(self):
        self.build_requires("generators/1.0.0@%s/stable" % self.user)
        self.build_requires("cmake/[>=3.15.3]@%s/stable" % self.user)

    def requirements(self):
        self.requires("libffi/3.3-rc0@%s/stable" % self.user)

    def source(self):
        tools.get("https://github.com/kcat/openal-soft/archive/openal-soft-%s.tar.gz" % self.version)

    def build(self):
        cmake = CMake(self, generator="Ninja")
        cmake.configure(source_folder="openal-soft-openal-soft-%s" % self.version)
        cmake.build()
        cmake.install()

#!/usr/bin/env python

from conans import ConanFile, tools, Meson
import os


class LibvaMesaDriverConan(ConanFile):
    name = "libva-mesa-driver"
    version = "2.3.0"
    license = "MIT"
    url = "https://github.com/prozum/conan-intel-vaapi-driver.git"
    description = "VA-API user mode driver for Intel GEN Graphics family"
    settings = "os", "arch", "compiler", "build_type"
    generators = "env"

    def requirements(self):
        self.requires("env-generator/0.1@%s/%s" % (self.user, self.channel))
        self.requires("libdrm/2.4.96@%s/stable" % self.user)
        self.requires("libva/2.3.0@%s/stable" % self.user)

    def source(self):
        tools.get("https://github.com/intel/intel-vaapi-driver/archive/%s.tar.gz" % self.version)

    def build(self):
        args = ["-Ddriverdir=" + os.path.join(self.package_folder, "lib", "dri")]
        meson = Meson(self)
        meson.configure(source_folder="intel-vaapi-driver-" + self.version, args=args)
        meson.install()

    def package_info(self):
        self.cpp_info.libs = tools.collect_libs(self)
        self.env_info.LIBVA_DRIVERS_PATH.append(os.path.join(self.package_folder, "lib", "dri"))

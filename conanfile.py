#!/usr/bin/env python
# -*- coding: utf-8 -*-

from conans import ConanFile, Meson, tools
import os


class LibVaConan(ConanFile):
    name = "libva"
    version = "2.3.0"
    description = "Libva is an implementation for VA-API (VIdeo Acceleration API)"
    default_user = "bincrafters"
    default_channel = "stable"
    url = "https://github.com/bincrafters/conan-" + name
    author = "BinCrafters <bincrafters@gmail.com>"
    license = "MIT"
    exports = ["LICENSE.md"]
    settings = "os", "arch", "compiler", "build_type"

    def requirements(self):
        self.requires("libdrm/2.4.96@%s/%s" % (self.user, self.channel))

    def source(self):
        tools.get("https://github.com/intel/libva/archive/%s.tar.gz" % self.version)

    def build(self):
        args = ["--libdir=lib"]
        meson = Meson(self)
        meson.configure(source_folder="libva-" + self.version, args=args, pkg_config_paths=os.environ["PKG_CONFIG_PATH"].split(":"))
        meson.build()
        meson.install()

    def package_info(self):
        self.cpp_info.libs = tools.collect_libs(self)
        self.env_info.PKG_CONFIG_PATH.append(os.path.join(self.package_folder, "lib", "pkgconfig"))
        for file in os.listdir(os.path.join(self.package_folder, "lib", "pkgconfig")):
            setattr(self.env_info, "PKG_CONFIG_%s_PREFIX" % file[:-3].replace(".", "_").replace("-", "_").upper(), self.package_folder)
        self.env_info.PATH.append(os.path.join(self.package_folder, "bin"))

#!/usr/bin/env python
# -*- coding: utf-8 -*-
from conans import ConanFile, CMake, tools
import os

def get_version():
    git = tools.Git()
    try:
        if git.get_tag():
            return git.get_tag()
        else:
            return git.get_branch()
    except:
        return None

class GstreamerColorizerConan(ConanFile):
    name = "gstreamer-colorizer"
    license = "LGPL"
    version = get_version()
    description = "Plugin to colorize 16 bit grayscale depth images with a color map"
    url = "https://aivero.com"
    settings = "os", "arch", "compiler", "build_type"
    exports_sources = ["CMakeLists.txt", "src/*"]
    generators = "env"
    gst_version = "1.16.0"

    def requirements(self):
        self.requires("env-generator/0.1@%s/stable" % self.user)
        self.requires("gstreamer/%s@%s/stable" % (self.gst_version, self.user))
        self.requires("gstreamer-plugins-base/%s@%s/stable" % (self.gst_version, self.user))

    def build(self):
        env = {
            "GIT_PKG_VER": "%s" % self.version,
        }
        with tools.environment_append(env):
            cmake = CMake(self)
            cmake.configure()
            cmake.build()
            cmake.install()

    def package(self):
        if self.settings.build_type == "Debug":
            self.copy("*gstcolorizer.c", "src")
            self.copy("*gstcolorizer.h", "src")

    def package_info(self):
        self.cpp_info.includedirs = ["include/gstreamer-1.0"]
        self.cpp_info.libs = tools.collect_libs(self)
        self.cpp_info.srcdirs.append("src")
        self.env_info.GST_PLUGIN_PATH.append(os.path.join(
            self.package_folder, "lib", "gstreamer-1.0"))

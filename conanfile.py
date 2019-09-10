from conans import ConanFile, Meson, tools
import os

def get_version():
    git = tools.Git()
    try:
        tag = git.get_tag()
        return tag if tag else "1.16.0"
    except:
        return None

class GStreamerDevtoolsConan(ConanFile):
    name = "gstreamer-devtools"
    version = get_version()
    url = "https://gitlab.com/aivero/public/conan/conan-" + name
    description = "Development and debugging tools for GStreamer"
    license = "https://gitlab.freedesktop.org/gstreamer/gstreamer/raw/master/COPYING"
    settings = "os", "arch", "compiler", "build_type"
    generators = "env"

    def requirements(self):
        self.requires("env-generator/0.1@%s/stable" % self.user)
        self.requires("glib/2.58.1@%s/stable" % self.user)
        self.requires("json-glib/1.4.4@%s/stable" % self.user)
        self.requires("gstreamer/%s@%s/stable" % (self.version, self.user))
        self.requires("gstreamer-plugins-base/%s@%s/stable" % (self.version, self.user))

    def source(self):
        tools.get("https://github.com/GStreamer/gst-devtools/archive/%s.tar.gz" % self.version)

    def build(self):
        args = ["--auto-features=disabled"]
        meson = Meson(self)
        meson.configure(source_folder="gst-devtools-" + self.version, args=args)
        meson.install()

    def package(self):
        if self.settings.build_type == "Debug":
            self.copy("*.c", "src")
            self.copy("*.h", "src")

    def package_info(self):
        self.cpp_info.libs = tools.collect_libs(self)
        self.cpp_info.srcdirs.append("src")

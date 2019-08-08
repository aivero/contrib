from conans import ConanFile, Meson, tools
import os

class GStreamerVaapiConan(ConanFile):
    name = "gstreamer-vaapi"
    version = "1.16.0"
    url = "https://gitlab.com/aivero/public/conan/conan-" + name
    description = "Hardware-accelerated video decoding, encoding and processing on Intel graphics through VA-API"
    license = "https://gitlab.freedesktop.org/gstreamer/gstreamer/raw/master/COPYING"
    settings = "os", "arch", "compiler", "build_type"
    generators = "env"

    def requirements(self):
        self.requires("env-generator/0.1@%s/%s" % (self.user, self.channel))
        self.requires("glib/2.58.1@%s/%s" % (self.user, self.channel))
        self.requires("gstreamer/%s@%s/%s" % (self.version, self.user, self.channel))
        self.requires("gstreamer-plugins-base/%s@%s/%s" % (self.version, self.user, self.channel))
        self.requires("gstreamer-plugins-bad/%s@%s/%s" % (self.version, self.user, self.channel))
        self.requires("libva/2.3.0@%s/%s" % (self.user, self.channel))

    def source(self):
        tools.get("https://github.com/GStreamer/gstreamer-vaapi/archive/%s.tar.gz" % self.version)

    def build(self):
        args = ["--auto-features=disabled"]
        meson = Meson(self)
        meson.configure(source_folder="gstreamer-vaapi-" + self.version)
        meson.install()

    def package(self):
        if self.settings.build_type == "Debug":
            self.copy("*.c", "src")
            self.copy("*.h", "src")

    def package_info(self):
        self.cpp_info.libs = tools.collect_libs(self)
        self.cpp_info.srcdirs.append("src")
        self.env_info.GST_PLUGIN_PATH.append(os.path.join(self.package_folder, "lib", "gstreamer-1.0"))

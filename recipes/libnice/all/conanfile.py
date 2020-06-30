import os

from conans import ConanFile, Meson, tools


class LibNiceConan(ConanFile):
    description = "An implementation of the IETF's Interactive Connectivity Establishment (ICE) standard"
    license = "LGPL"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    options = {"gstreamer": [True, False]}
    default_options = "gstreamer=True"

    def build_requirements(self):
        self.build_requires("generators/1.0.0")
        self.build_requires("meson/[^0.51.2]")

    def requirements(self):
        self.requires("glib/[^2.62.0]")
        self.requires("openssl/[^1.1.1b]")
        if self.options.gstreamer:
            self.requires("gstreamer-plugins-base/%s" % (self.gst_version))

    def source(self):
        tools.get("https://github.com/libnice/libnice/archive/%s.tar.gz" % self.version)

    def build(self):
        args = ["--auto-features=disabled"]
        args.append("-Dgstreamer=" + ("enabled" if self.options.gstreamer else "disabled"))
        meson = Meson(self)
        meson.configure(source_folder="%s-%s" % (self.name, self.version), args=args, pkg_config_paths=os.environ["PKG_CONFIG_PATH"].split(":"))
        meson.install()

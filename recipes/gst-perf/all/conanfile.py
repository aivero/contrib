import os
import shutil

from conans import *


class GStreamerPerfConan(ConanFile):
    description = "Performance Evaluation tool for Gstreamer"
    license = "LGPL"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    )
    requires = (
        "glib/[^2.62.0]",
        "gstreamer/[^%s]" % (self.version),
    build_requires = (
        "generators/[^1.0.0]",
        "autotools/[^1.0.0]",
        "automake/[^1.16.1]",
        "autoconf/[^2.69]",
    )

    def source(self):
        git = tools.Git()
        git.clone("https://github.com/RidgeRun/gst-perf.git", "v%s" % self.version)

    def build(self):
        self.run("sh autogen.sh")
        autotools = AutoToolsBuildEnvironment(self)
        autotools.configure()
        autotools.make()
        autotools.install()

    def package(self):
        self.copy(pattern="*.so", dst=os.path.join(self.package_folder, "lib", "gstreamer-1.0"), keep_path=False)


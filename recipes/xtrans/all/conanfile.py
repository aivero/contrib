from conans import *


class XtransConan(ConanFile):
    description = "X transport library"
    license = "MIT"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    build_requires = ("generators/1.0.0",)

    def source(self):
        tools.get("https://xorg.freedesktop.org/releases/individual/lib/xtrans-%s.tar.gz" % self.version)

    def build(self):
        autotools = AutoToolsBuildEnvironment(self)
        with tools.chdir("%s-%s" % (self.name, self.version)):
            autotools.configure()
            autotools.install()

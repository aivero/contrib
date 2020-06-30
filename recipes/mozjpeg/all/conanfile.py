import os

from conans import AutoToolsBuildEnvironment, ConanFile, tools


class MozjpegConan(ConanFile):
    description = "JPEG image codec with accelerated baseline compression and decompression"
    license = "custom"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}

    def build_requirements(self):
        self.build_requires("generators/1.0.0")
        self.build_requires("autotools/1.0.0")
        self.build_requires("yasm/[^1.3.0]")
        self.build_requires("cmake/[^3.15.3]")

    def source(self):
        tools.get("https://github.com/mozilla/mozjpeg/archive/v{0}.tar.gz".format(self.version))

    def build(self):
        args = [
            "--disable-static",
        ]

        with tools.chdir("%s-%s" % (self.name, self.version)):
            self.run("autoreconf -i")
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure(args=args)
            autotools.make()
            autotools.install()

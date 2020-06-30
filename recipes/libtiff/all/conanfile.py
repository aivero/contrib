from conans import AutoToolsBuildEnvironment, ConanFile, tools


class LibtiffConan(ConanFile):
    description = "Library for manipulation of TIFF images"
    license = "custom"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}

    def build_requirements(self):
        self.build_requires("generators/1.0.0@%s/stable" % self.user)
        self.build_requires("mesa/20.0.0@%s/stable" % self.user)

    def requirements(self):
        self.requires("zlib/[>=1.2.11]@%s/stable" % self.user)
        self.requires("libjpeg-turbo/[>=2.0.4]@%s/stable" % self.user)

    def source(self):
        tools.get("https://download.osgeo.org/libtiff/tiff-%s.tar.gz" % self.version)

    def build(self):
        args = ["--disable-static"]
        with tools.chdir("tiff-%s" % self.version):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure(args=args)
            autotools.make()
            autotools.install()

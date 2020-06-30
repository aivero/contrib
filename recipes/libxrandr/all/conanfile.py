from conans import AutoToolsBuildEnvironment, ConanFile, tools


class LibxrandrConan(ConanFile):
    description = "X11 RandR extension library"
    license = "MIT"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}

    def build_requirements(self):
        self.build_requires("generators/1.0.0@%s/stable" % self.user)
        self.build_requires("pkgconf/[^1.6.3]@%s/stable" % self.user)
        self.build_requires("xorg-util-macros/[^1.19.1]@%s/stable" % self.user)

    def requirements(self):
        self.requires("libx11/[^1.6.8]@%s/stable" % self.user)
        self.requires("libxrender/[^0.9.10]@%s/stable" % self.user)
        self.requires("libxext/[^1.3.4]@%s/stable" % self.user)

    def source(self):
        tools.get("https://xorg.freedesktop.org/releases/individual/lib/libXrandr-%s.tar.gz" % self.version)

    def build(self):
        args = ["--disable-static"]
        autotools = AutoToolsBuildEnvironment(self)
        with tools.chdir("libXrandr-" + self.version):
            autotools.configure(args=args)
            autotools.install()

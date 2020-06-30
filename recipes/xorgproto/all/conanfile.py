from conans import ConanFile, Meson, tools


class XorgProtoConan(ConanFile):
    description = "combined X.Org X11 Protocol headers"
    license = "custom"
    settings = "os", "arch", "compiler", "build_type"

    def build_requirements(self):
        self.build_requires("generators/1.0.0@%s/stable" % self.user)
        self.build_requires("meson/[>=0.51.2]@%s/stable" % self.user)
        self.build_requires("xorg-util-macros/[>=1.19.1]@%s/stable" % self.user)

    def source(self):
        tools.get(
            "https://xorg.freedesktop.org/archive/individual/proto/xorgproto-%s.tar.bz2"
            % self.version
        )

    def build(self):
        args = ["--auto-features=disabled"]
        meson = Meson(self)
        meson.configure(
            source_folder="%s-%s" % (self.name, self.version),
            args=args,
            pkg_config_paths=os.environ["PKG_CONFIG_PATH"].split(":"),
        )
        meson.install()

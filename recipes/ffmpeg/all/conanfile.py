from conans import AutoToolsBuildEnvironment, ConanFile, tools


class FFMpegConan(ConanFile):
    description = "A complete, cross-platform solution to record, convert and stream audio and video"
    license = "GPL3"
    settings = "os", "arch", "compiler", "build_type"

    def build_requirements(self):
        self.build_requires("generators/1.0.0@%s/stable" % self.user)
        self.build_requires("yasm/[>=1.3.0]@%s/stable" % self.user)

    def source(self):
        tools.get("http://ffmpeg.org/releases/ffmpeg-%s.tar.bz2" % self.version)

    def build(self):
        args = [
            "--disable-static",
            "--enable-shared",
            "--disable-doc",
            "--disable-programs",
        ]
        if self.settings.build_type == "Debug":
            args.extend(
                [
                    "--disable-optimizations",
                    "--disable-mmx",
                    "--disable-stripping",
                    "--enable-debug",
                ]
            )
        with tools.chdir("%s-%s" % (self.name, self.version)):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure(args=args)
            autotools.make()
            autotools.install()

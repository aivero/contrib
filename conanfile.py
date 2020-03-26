from conans import AutoToolsBuildEnvironment, ConanFile, tools


class GiflibConan(ConanFile):
    name = "giflib"
    version = tools.get_env("GIT_TAG", "5.2.1")
    description = "Library for reading and writing gif images"
    url = "https://gitlab.com/aivero/public/conan/conan-" + name
    license = "custom"
    settings = "os", "compiler", "build_type", "arch"

    def build_requirements(self):
        self.build_requires("generators/1.0.0@%s/stable" % self.user)
        self.build_requires("autotools/[>=1.0.0]@%s/stable" % self.user)

    def source(self):
        tools.get("https://downloads.sourceforge.net/project/giflib/giflib-%s.tar.gz" % self.version)

    def build(self):
        autotools = AutoToolsBuildEnvironment(self)
        with tools.chdir("%s-%s" % (self.name, self.version)):
            autotools.make()
            autotools.install(["PREFIX=" + self.package_folder])

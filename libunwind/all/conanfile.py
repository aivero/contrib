from conans import AutoToolsBuildEnvironment, ConanFile, tools


class LibunwindConan(ConanFile):
    name = "libunwind"
    version = tools.get_env("GIT_TAG", "1.3.1")
    settings = "os", "compiler", "build_type", "arch"
    license = "MIT"
    description = "Portable and efficient C programming interface (API) to determine the call-chain of a programs"

    def build_requirements(self):
        self.build_requires("generators/1.0.0@%s/stable" % self.user)
        self.build_requires("autotools/[>=1.0.0]@%s/stable" % self.user)

    def source(self):
        tools.get("https://download.savannah.gnu.org/releases/libunwind/libunwind-%s.tar.gz" % self.version)

    def build(self):
        args = [
            "--disable-static",
        ]
        with tools.chdir("%s-%s" % (self.name, self.version)):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure(args=args)
            autotools.make()
            autotools.install()

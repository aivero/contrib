from conans import AutoToolsBuildEnvironment, ConanFile, tools


def get_version():
    git = tools.Git()
    try:
        tag = git.get_tag()
        return tag if tag else "3.3-rc0"
    except:
        return None


class LibffiConan(ConanFile):
    name = "libffi"
    version = get_version()
    settings = "os", "compiler", "build_type", "arch"
    url = "https://gitlab.com/aivero/public/conan/conan-" + name
    license = "MIT"
    description = (
        "A portable, high level programming interface to various calling conventions"
    )
    generators = "env"

    def build_requirements(self):
        self.build_requires("autotools/[>=1.0.0]@%s/stable" % self.user)

    def requirements(self):
        self.requires("env-generator/[>=1.0.0]@%s/stable" % self.user)

    def source(self):
        tools.get("https://github.com/libffi/libffi/archive/v%s.tar.gz" % self.version)

    def build(self):
        args = [
            "--quiet",
            "--disable-debug",
            "--disable-dependency-tracking",
            "--disable-docs",
            "--disable-static",
            "--enable-shared",
        ]
        with tools.chdir("%s-%s" % (self.name, self.version)):
            self.run("sh autogen.sh")
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure(args=args)
            autotools.make()
            autotools.install()

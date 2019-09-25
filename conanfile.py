from conans import ConanFile, tools, AutoToolsBuildEnvironment
import os

def get_version():
    git = tools.Git()
    try:
        tag = git.get_tag()
        return tag if tag else "2.4.6"
    except:
        return None

class LibtoolConan(ConanFile):
    name = "libtool"
    version = get_version()
    settings = "os", "compiler", "build_type", "arch"
    url = "https://gitlab.com/aivero/public/conan/conan-" + name
    license = "GPL"
    description = "A generic library support script"
    exports = "libtool-prefix-fix.patch"
    generators = "env"

    def build_requirements(self):
        self.build_requires("env-generator/0.1@%s/stable" % self.user)
        self.build_requires("automake/1.16.1@%s/stable" % self.user)
        self.build_requires("help2man/1.47.11@%s/stable" % self.user)
        self.build_requires("texinfo/6.6@%s/stable" % self.user)

    def source(self):
        git = tools.Git(folder="%s-%s" % (self.name, self.version))
        git.clone("https://git.savannah.gnu.org/git/libtool.git", "v" + self.version)
        git = tools.Git(folder="gnulib")
        git.clone("https://git.savannah.gnu.org/git/gnulib.git")
        git = tools.Git(folder="gnulib-bootstrap")
        git.clone("https://github.com/gnulib-modules/bootstrap.git")
        tools.patch(patch_file="libtool-prefix-fix.patch", base_path="%s-%s" % (self.name, self.version))

    def build(self):
        with tools.chdir("%s-%s" % (self.name, self.version)):
            self.run("git submodule init")
            self.run("git config --local submodule.gnulib.url \"%s/gnulib\"" % self.source_folder)
            self.run("git config --local submodule.gl-mod/bootstrap.url \"%s/gnulib-bootstrap\"" % self.source_folder)
            self.run("git submodule update")
            self.run("./bootstrap")
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure()
            autotools.make()
            autotools.install()

    def package(self):
        if self.settings.build_type == "Debug":
            self.copy("*.c", "src")
            self.copy("*.h", "src")

    def package_info(self):
        self.cpp_info.srcdirs.append("src")
        self.env_info.LIBTOOL_PREFIX = self.package_folder
        self.env_info.LIBTOOL = os.path.join(self.package_folder, "bin", "libtool")
        self.env_info.LIBTOOLIZE = os.path.join(self.package_folder, "bin", "libtoolize")

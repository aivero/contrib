from conans import ConanFile, tools

def get_version():
    git = tools.Git()
    try:
        tag = git.get_tag()
        return tag if tag else "3.15.3"
    except:
        return None

class CMakeConan(ConanFile):
    name = "cmake"
    version = get_version()
    settings = "os", "compiler", "build_type", "arch"
    url = "https://gitlab.com/aivero/public/conan/conan-" + name
    license = "custom"
    description = "A cross-platform open-source make system"
    generators = "env"

    def requirements(self):
        self.requires("env-generator/0.1@%s/stable" % self.user)

    def source(self):
        tools.get("https://github.com/Kitware/CMake/releases/download/v{0}/cmake-{0}.tar.gz".format(self.version))

    def build(self):
        with tools.chdir("%s-%s" % (self.name, self.version)):
            self.run("./bootstrap --prefix=" + self.package_folder)
            self.run("make")

    def package(self):
        if self.settings.build_type == "Debug":
            self.copy("*.c", "src")
            self.copy("*.h", "src")

    def package_info(self):
        self.cpp_info.srcdirs.append("src")

import os

from conans import AutoToolsBuildEnvironment, ConanFile, tools


class SwigConan(ConanFile):
    description = "Generate scripting interfaces to C/C++ code"
    license = "custom"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}

    def build_requirements(self):
        self.build_requires("generators/1.0.0@%s/stable" % self.user)
        self.build_requires("gcc/[>=7.4.0]@%s/stable" % self.user)
        self.build_requires("python/[>=3.7.4]@%s/stable" % self.user)

    def source(self):
        tools.get("https://downloads.sourceforge.net/swig/swig-%s.tar.gz" % self.version)

    def build(self):
        env = {"PATH": tools.get_env("PATH") + os.path.pathsep + os.path.join(self.package_folder, "bin")}
        with tools.chdir("%s-%s" % (self.name, self.version)), tools.environment_append(env):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure()
            autotools.install()

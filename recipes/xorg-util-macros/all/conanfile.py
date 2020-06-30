import os

from conans import AutoToolsBuildEnvironment, ConanFile, tools


class LibXorgUtilMacrosConan\(ConanFile\):
    description = "X.Org Autotools macros"
    license = "custom"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}

    def build_requirements(self):
        self.build_requires("generators/1.0.0@%s/stable" % self.user)

    def source(self):
        tools.get("https://xorg.freedesktop.org/releases/individual/util/util-macros-%s.tar.gz" % self.version)

    def build(self):
        autotools = AutoToolsBuildEnvironment(self)
        with tools.chdir("util-macros-%s" % self.version):
            autotools.configure()
            autotools.install()

    def package_info(self):
        self.env_info.ACLOCAL_PATH.append(os.path.join(self.package_folder, "share", "aclocal"))

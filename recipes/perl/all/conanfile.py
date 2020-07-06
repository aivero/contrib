import os

from conans import *


class PerlConan(ConanFile):
    description = "A highly capable, feature-rich programming language"
    license = "GPL"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    exports = "link-m-pthread.patch"
    build_requires = ("cc/[^1.0.0]",)

    def source(self):
        tools.get(f"https://github.com/Perl/perl5/archive/v{self.version}.tar.gz")
        tools.patch(
            patch_file="link-m-pthread.patch", base_path=f"{self.name}5-{self.version}",
        )

    def build(self):
        args = [
            "-des",
            "-Dusethreads",
            "-Uusenm",
            "-Duseshrplib",
            "-Duselargefiles",
            "-Dprefix=" + self.package_folder,
            "-Dlddlflags='-shared'",
            "-Dldflags=''",
        ]
        with tools.chdir(f"{self.name}5-{self.version}"):
            autotools = AutoToolsBuildEnvironment(self)
            self.run("./Configure " + " ".join(args))
            autotools.make()
            autotools.install()

    def package_info(self):
        arch_conv = {"x86_64": "x86_64", "armv8": "aarch64"}
        platform = arch_conv[str(self.settings.arch_build)] + "-linux"
        self.env_info.PERL = "perl"
        self.env_info.PERL5LIB.append(os.path.join(self.package_folder, "lib", self.version))
        self.env_info.PERL5LIB.append(os.path.join(self.package_folder, "lib", self.version, platform + "-thread-multi"))

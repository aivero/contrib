import os

from conans import AutoToolsBuildEnvironment, ConanFile, tools


class PerlConan\(ConanFile\):
    description = "A highly capable, feature-rich programming language"
    license = "GPL"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    exports = "link-m-pthread.patch"

    def build_requirements(self):
        self.build_requires("generators/1.0.0@%s/stable" % self.user)
        self.build_requires("gcc/[>=7.4.0]@%s/stable" % self.user)

    def source(self):
        tools.get("https://github.com/Perl/perl5/archive/v%s.tar.gz" % self.version)
        tools.patch(
            patch_file="link-m-pthread.patch", base_path="%s5-%s" % (self.name, self.version),
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
        with tools.chdir("%s5-%s" % (self.name, self.version)):
            autotools = AutoToolsBuildEnvironment(self)
            self.run("./Configure " + " ".join(args))
            autotools.make()
            autotools.install()

    def package_info(self):
        arch_conv = {"x86_64": "x86_64", "armv8": "aarch64"}
        platform = "%s-linux" % arch_conv[str(self.settings.arch)]
        self.env_info.PERL = "perl"
        self.env_info.PERL5LIB.append(os.path.join(self.package_folder, "lib", self.version))
        self.env_info.PERL5LIB.append(os.path.join(self.package_folder, "lib", self.version, "%s-thread-multi" % platform))

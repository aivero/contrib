from build import *


class Perl(Recipe):
    description = "A highly capable, feature-rich programming language"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )
    exports = "link-m-pthread.patch"

    def source(self):
        self.get(f"https://github.com/Perl/perl5/archive/v{self.version}.tar.gz")
        self.patch("link-m-pthread.patch")

    def build(self):
        args = [
            f"-Dprefix={self.package_folder}",
            "-des",
            "-Dusethreads",
            "-Uusenm",
            "-Duseshrplib",
            "-Duselargefiles",
            "-Dldflags=''",
        ]
        self.exe("./Configure", args)
        self.make()

    def package_info(self):
        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch)]
        self.env_info.PERL = "perl"
        self.env_info.PERL5LIB.append(os.path.join(self.package_folder, "lib", self.version))
        self.env_info.PERL5LIB.append(
            os.path.join(self.package_folder, "lib", self.version, f"{arch}-linux-thread-multi")
        )

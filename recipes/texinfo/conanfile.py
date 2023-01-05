from build import *


class Texinfo(Recipe):
    description = "GNU documentation system for on-line information and printed output"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "perl/[^5.30.0]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/pub/gnu/texinfo/texinfo-{self.version}.tar.xz")

    def package_info(self):
        self.env_info.MAKEINFO = os.path.join(self.package_folder, "bin", "makeinfo")
        self.env_info.PERL5LIB.append(os.path.join(self.package_folder, "share", "texinfo"))
        for mod in ["libintl-perl", "Text-Unidecode", "Unicode-EastAsianWidth"]:
            self.env_info.PERL5LIB.append(
                os.path.join(self.package_folder, "share", "texinfo", "lib", mod, "lib")
            )

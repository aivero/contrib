from build import *


class Gettext(Recipe):
    description = "GNU internationalization library"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/pub/gnu/gettext/gettext-{self.version}.tar.gz")

    def package_info(self):
        self.env_info.gettext_datadir.append(os.path.join(self.package_folder, "share", "gettext"))

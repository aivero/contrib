from build import *


class M4(Recipe):
    description = "The GNU macro processor"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/m4/m4-{self.version}.tar.gz")

    def package_info(self):
        self.env_info.M4 = os.path.join(self.package_folder, "bin", "m4")

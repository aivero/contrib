from build import *


class Pkgconf(Recipe):
    description = "Package compiler and linker metadata toolkit"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "automake/[^1.16.1]",
        "libtool/[^2.4.6]",
    )

    def build(self):
        self.autotools()

    def source(self):
        self.get(f"https://github.com/pkgconf/pkgconf/archive/pkgconf-{self.version}.tar.gz")

    def package(self):
        os.symlink("pkgconf", os.path.join(self.package_folder, "bin", "pkg-config"))

    def package_info(self):
        self.env_info.PKG_CONFIG = os.path.join(self.package_folder, "bin", "pkgconf")
        # Support system pkgconfig files
        # if self.settings.os == "Linux":
        #    self.env_info.PKG_CONFIG_SYSTEM_PATH.append("/usr/share/pkgconfig")
        #    if self.settings.arch_build == "x86_64":
        #        self.env_info.PKG_CONFIG_SYSTEM_PATH.append("/usr/lib/x86_64-linux-gnu/pkgconfig")
        #    if self.settings.arch_build == "armv8":
        #        self.env_info.PKG_CONFIG_SYSTEM_PATH.append("/usr/lib/aarch64-linux-gnu/pkgconfig")

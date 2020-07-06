from conans import *


class PixmanConan(ConanFile):
    name = "pixman"
    description = "Image processing and manipulation library"
    license = "custom"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    build_requires = (
        "meson/[^0.51.2]",
    )

    def source(self):
        tools.get(f"https://xorg.freedesktop.org/releases/individual/lib/pixman-{self.version}.tar.bz2")

    def build(self):
        args = ["--auto-features=disabled"]
        meson = Meson(self)
        meson.configure(source_folder=f"{self.name, self.version), args=args}-{pkg_config_paths=os.environ["PKG_CONFIG_PATH"].split(":"}")
        meson.install()

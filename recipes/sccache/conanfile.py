from build import *


class SccacheRecipe(RustRecipe):
    settings = RustRecipe.settings + ("compiler",)
    description = "Development and debugging tools for GStreamer"
    license = "Apache"
    build_requires = ("pkgconf/[^1.7.3]",)
    requires = ()

    def requirements(self):
        # Rust is not compatible between major or minor releases
        self.requires(f"rustc/[~{self.settings.rust}]")

    def source(self):
        self.get(f"https://github.com/mozilla/sccache/archive/{self.version}.tar.gz")

    def package_info(self):
        self.env_info.RUSTC_WRAPPER = "sccache"

    def build(self):
        self.cargo(test=False)

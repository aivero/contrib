from build import *


class RustRecipe(RustRecipe):
    description = "Virtual rust package"
    license = "MIT"
    requires = ("pkgconf/[^1.7.3]",)

    def requirements(self):
        self.requires(f"rustc/[~{self.settings.rust}]")

    def package_info(self):
        self.env_info.CARGO_REGISTRIES_CRATES_IO_PROTOCOL = "sparse"

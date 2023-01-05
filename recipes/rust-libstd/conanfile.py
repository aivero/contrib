from build import *


class RustLibstd(RustRecipe):
    settings = RustRecipe.settings + ("compiler",)
    description = "Rust libstd"
    license = "MIT"
    keep_imports = True

    def requirements(self):
        # Rust is not compatible between major or minor releases
        self.requires(f"rustc/[~{self.settings.rust}]", "private")

    def imports(self):
        self.copy("*libstd-*.so", excludes="*rustlib*")

    def package(self):
        self.copy("*libstd-*.so")

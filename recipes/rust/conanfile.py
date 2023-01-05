from build import *


class RustRecipe(RustRecipe):
    settings = RustRecipe.settings + ("compiler",)
    description = "Virtual rust package"
    license = "MIT"
    requires = (
        "sccache/[^0.2.14]",
        "pkgconf/[^1.7.3]",
    )

    def package_info(self):
        # if not os.path.exists(cache_folder):
        #    os.makedirs(cache_folder)
        self.env_info.RUSTFLAGS = "-g"
        self.env_info.CARGO_TARGET_DIR = os.path.join(self.conan_home, "cache", "cargo")

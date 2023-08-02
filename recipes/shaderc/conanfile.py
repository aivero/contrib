from build import *


class Shaderc(PythonRecipe):
    description = "A collection of tools, libraries, and tests for Vulkan shader compilation."
    license = "Apache"
    options = {}
    default_options = {}
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
        "git/[^2.29.1]",
    )

    def build_requirements(self):
        self.build_requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/google/shaderc/archive/v{self.version}.tar.gz")
        self.exe("./utils/git-sync-deps")

    def package_info(self):
        self.env_info.SHADERC_LIB_DIR.append(os.path.join(self.package_folder, "lib"))

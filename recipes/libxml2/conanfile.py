from build import *


class Libxml2Recipe(Recipe):
    description = "XML parsing library, version 2"
    license = "MIT"
    settings = "build_type", "compiler", "arch_build", "os_build", "libc_build", "python"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "zlib/[^1.2.11]",
    )

    def build_requirements(self):
        self.build_requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://gitlab.gnome.org/GNOME/libxml2/-/archive/v{self.version}/libxml2-v{self.version}.tar.bz2")

    def build(self):
        os.environ["with_python_install_dir"] = os.path.join(self.package_folder, "lib", f"python{self.settings.python}", "site-packages")
        self.autotools()

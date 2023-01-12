from build import *


class Gdb(PythonRecipe):
    description = "the GNU Project debugger, allows you to see what is going on inside another program while it executes"
    license = "GPL"

    build_requires = (
        "autotools/[^1.0.0]",
        "bison/[^3.3]",
        "cc/[^1.0.0]",
        "flex/[^2.6.4]",
    )
    requires = (
        "gmp/[^6.2.0]",
        "libelf/[^0.8.13]",
    )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        version = ".".join(self.version.split(".")[:2])
        self.get(f"https://ftp.gnu.org/gnu/gdb/gdb-{version}.tar.xz")

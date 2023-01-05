from build import *


class Tcl(Recipe):
    description = "The Tcl scripting language"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "zlib/[^1.2.11]",
    )

    def source(self):
        self.get(f"https://downloads.sourceforge.net/sourceforge/tcl/tcl{self.version}-src.tar.gz")

    def build(self):
        os.environ["CFLAGS"] += " -lm"
        args = [
            "--enable-threads",
            "--enable-64bit",
        ]
        self.autotools(args, os.path.join(self.src, "unix"))

from build import *


class Readline(Recipe):
    description = "GNU readline library"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )
    requires = ("ncurses/[^6.1]",)

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/readline/readline-{self.version}.tar.gz")

    def build(self):
        args = [
            "SHLIB_LIBS=-lncurses",
        ]
        self.autotools(make_args=args)

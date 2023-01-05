from build import *


class Libedit(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "System V Release 4.0 curses emulation library"
    license = "BSDBBBBB"
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")
    requires = ("ncurses/[^6.1]",)

    def source(self):
        self.get(f"https://thrysoee.dk/editline/libedit-{self.version}.tar.gz")

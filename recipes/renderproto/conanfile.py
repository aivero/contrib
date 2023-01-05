from build import *


class Renderproto(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "X11 Render extension wire protocol"
    license = "MIT"
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")

    def source(self):
        self.get(
            f"https://xorg.freedesktop.org/releases/individual/proto/renderproto-{self.version}.tar.gz"
        )
        with tools.chdir(self.src):
            os.remove("config.guess")
            os.remove("config.sub")
            tools.download(
                "http://git.savannah.gnu.org/gitweb/?p=config.git;a=blob_plain;f=config.guess;hb=HEAD",
                "config.guess",
            )
            tools.download(
                "http://git.savannah.gnu.org/gitweb/?p=config.git;a=blob_plain;f=config.sub;hb=HEAD",
                "config.sub",
            )

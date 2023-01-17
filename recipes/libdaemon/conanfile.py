from build import *

class LibDaemon(Recipe):
    description = "a lightweight C library that eases the writing of UNIX daemons"
    license = "LGPL"
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")
    
    def source(self):
        self.get(f"http://0pointer.de/lennart/projects/libdaemon/libdaemon-{self.version}.tar.gz")
        self.download(f"https://git.savannah.gnu.org/gitweb/?p=config.git;a=blob_plain;f=config.guess;hb=HEAD", "config.guess")

    def build(self):
        args = [
            "--disable-examples"
        ]
        self.autotools(args)
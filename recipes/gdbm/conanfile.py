from build import *

class Gdbm(Recipe):
    description = ("gdbm is a library of database functions that uses "
                   "extensible hashing and work similar to "
                   "the standard UNIX dbm. "
                   "These routines are provided to a programmer needing "
                   "to create and manipulate a hashed database.")
    license = "GPL"
    build_requires = (
        "autotools/[^1.0.0]",
        "bison/[^3.7.6]",
        "flex/[^2.6.4]",
        "cc/[^1.0.0]",
    )
    requires = (
        "libiconv/[^1.16]",
        "readline/[^8.1]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/gdbm/gdbm-{self.version}.tar.gz")

    def build(self):
        self.autotools()

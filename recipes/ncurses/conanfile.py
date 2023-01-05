from build import *


class Ncurses(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "System V Release 4.0 curses emulation library"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )

    def source(self):
        self.get(f"https://ftp.gnu.org/pub/gnu/ncurses/ncurses-{self.version}.tar.gz")

    def build(self):
        args = [
            f'--with-pkg-config-libdir={os.path.join(self.package_folder, "lib", "pkgconfig")}',
            "--enable-overwrite",
            "--without-debug",
            "--without-cxx-binding",
            "--enable-pc-files",
        ]
        if self.options.shared:
            args.append("--with-shared")
            args.append("--without-normal")
        self.autotools(args)

    def package_info(self):
        self.env_info.TERMINFO = os.path.join(self.package_folder, "share", "terminfo")

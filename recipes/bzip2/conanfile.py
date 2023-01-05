from build import *


class Bzip2(Recipe):
    description = "A high-quality data compression program"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )

    def source(self):
        self.get(f"https://sourceware.org/pub/bzip2/bzip2-{self.version}.tar.gz")

    def build(self):
        args = [
            f"PREFIX={self.package_folder}",
            f"CC=cc {os.environ['CFLAGS']}",
        ]
        self.make(args)
        if self.options.shared:
            args.extend(["-f", "Makefile-libbz2_so"])
        self.make(args, target="all")

    def package(self):
        if self.options.shared:
            self.copy("*.so*", dst="lib", keep_path=False, links=True)
            os.symlink("libbz2.so.1.0", os.path.join(self.package_folder, "lib", "libbz2.so"))
            os.remove(os.path.join(self.package_folder, "lib", "libbz2.a"))
        with tools.chdir(os.path.join(self.package_folder, "bin")):
            for source, link in (
                ("bzdiff", "bzcmp"),
                ("bzgrep", "bzegrep"),
                ("bzgrep", "bzfgrep"),
                ("bzmore", "bzless"),
            ):
                os.remove(link)
                os.symlink(source, link)

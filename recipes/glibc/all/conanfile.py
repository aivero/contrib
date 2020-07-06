from conans import *


class GlibcConan(ConanFile):
    name = "glibc"
    description = "GNU C Library"
    license = "GPL"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}

    def source(self):
        tools.get(f"https://ftp.gnu.org/gnu/glibc/glibc-{self.version}.tar.xz")

    def build(self):
        args = [
            "--enable-add-ons",
            "--enable-bind-now",
            "--enable-cet",
            "--enable-lock-elision",
            "--enable-multi-arch",
            "--enable-stack-protector=strong",
            "--enable-stackguard-randomization",
            "--enable-static-pie",
            # "--enable-systemtap",
            "--disable-profile",
            "--disable-werror",
            "--host=x86_64-pc-linux-gnu",
        ]
        vars = {
            "CFLAGS": "-O2 -U_FORTIFY_SOURCE=2 -fno-plt",
        }
        autotools = AutoToolsBuildEnvironment(self)
        autotools.configure(args=args, vars=vars, configure_dir=f"{self.name}-{self.version}")
        autotools.make()
        autotools.install()

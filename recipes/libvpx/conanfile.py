from build import *


class LibVpx(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "WebM VP8/VP9 Codec SDK"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "yasm/[^1.3.0]",
    )

    def source(self):
        self.get(f"https://github.com/webmproject/libvpx/archive/v{self.version}.tar.gz")

    def build(self):
        args = [
            "--disable-examples",
            "--disable-install-docs",
            "--disable-install-srcs",
            "--enable-vp9-highbitdepth",
        ]
        if self.options.shared:
            args.append("--enable-shared")
            args.append("--disable-static")
        os.environ["STRIP"] = "no"
        os.environ["LDFLAGS"] = "-lpthread"
        self.autotools(args)

from build import *


class FFMpeg(Recipe):
    description = (
        "A complete, cross-platform solution to record, convert and stream audio and video"
    )
    license = "LGPL"
    settings = Recipe.settings + ("compiler",)
    options = {"nonfree": [True, False], "v4l2_m2m": [True, False], "shared": [True, False]}
    default_options = ("nonfree=False", "v4l2_m2m=True", "shared=True")
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "yasm/[^1.3.0]",
    )

    def source(self):
        self.get(f"http://ffmpeg.org/releases/ffmpeg-{self.version}.tar.bz2")

    def build(self):
        args = [
            "--disable-doc",
            "--disable-programs",
            "--cc=cc",
            "--cxx=c++",
        ]

        # additional args for rpi.4 64-bit
        if self.options.nonfree:
            args += ["--enable-nonfree"]
        if self.options.v4l2_m2m:
            args += ["--enable-v4l2_m2m"]

        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch)]
        args += [f"--arch={arch}"]

        self.autotools(args)

from build import *


class GstSharkRecipe(GstRecipe):
    settings = GstRecipe.settings + ("compiler",)
    description = "GstShark is a front-end for GStreamer traces "
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = ("graphviz/[^2.42.1]",)

    def requirements(self):
        self.requires(f"gst/[~{self.settings.gstreamer}]")

    def source(self):
        self.get(f"https://github.com/RidgeRun/gst-shark/archive/v{self.version}.tar.gz")
        git = tools.Git(folder=os.path.join("gst-shark-" + self.version, "common"))
        git.clone("git://anongit.freedesktop.org/gstreamer/common", "master")

    def build(self):
        args = [
            "--disable-gtk-doc",
        ]
        self.autotools(args)

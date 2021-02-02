from build import *


class GstSharkRecipe(Recipe):
    description = "GstShark is a front-end for GStreamer traces "
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]", 
        "autotools/[^1.0.0]",
    )
    requires = (
        "gstreamer/[^1.18]",
        "graphviz/[^2.42.1]",
    )

    def source(self):
        self.get(f"https://github.com/RidgeRun/gst-shark/archive/v{self.version}.tar.gz")
        git = tools.Git(folder=os.path.join("gst-shark-" + self.version, "common"))
        git.clone("git://anongit.freedesktop.org/gstreamer/common", "master")

    def build(self):
        args = [
            "--disable-gtk-doc",
        ]
        self.autotools(args)
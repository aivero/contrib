from conans.model.requires import Requirement
from build import *


class GstPerfRecipe(GstRecipe):
    settings = GstRecipe.settings + ("compiler",)
    description = "Performance Evaluation tool for Gstreamer"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
        "automake/[^1.16.1]",
        "autoconf/[^2.69]",
    )

    def requirements(self):
        self.requires(f"gst/[~{self.settings.gstreamer}]")

    def source(self):
        self.get(f"https://github.com/RidgeRun/gst-perf/archive/v{self.version}.tar.gz")

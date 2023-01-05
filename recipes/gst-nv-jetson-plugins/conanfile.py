from build import *


class GstNvJetsonPlugins(Recipe):
    description = "Demo conan package"
    license = "MIT"
    exports_sources = ["lib/gstreamer-1.0/*.so"]

    def requirements(self):
        self.requires(f"nv-jetson-drivers/[^{self.version}]")

    def package(self):
        self.copy(pattern="*.so", excludes="*libgstnvvideo4linux2.so*")

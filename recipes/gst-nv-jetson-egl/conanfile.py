from build import *


class GstNvJetsonEgl(GstRecipe):
    settings = GstRecipe.settings + ("compiler",)
    description = "NVIDIA jetson egl element"
    license = "LGPL"
    options = {"jetson": ["Nano", "TX2", "Xavier"]}
    default_options = ("jetson=TX2",)
    build_requires = (
        "cc/[^1.0.0]",
        "pkgconf/[^1.6.3]",
    )

    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")
        self.requires(f"nv-jetson-drivers/[^{self.version}]")
        self.requires(f"nv-jetson-v4l2/[^{self.version}]")

    def source(self):
        if self.options.jetson in ("TX2", "Xavier"):
            self.get(
                f"https://developer.nvidia.com/embedded/dlc/r{self.version}_Release_v1.0/TX2-AGX/sources/public_sources.tbz2".replace(
                    ".", "-"
                )
            )
        elif self.options.jetson == "Nano":
            self.get(
                f"https://developer.nvidia.com/embedded/dlc/r{self.version}_Release_v1.0/Nano-TX1/sources/public_sources.tbz2".replace(
                    ".", "-"
                )
            )
        else:
            raise KeyError("Unknown option: " + self.options.jetson)
        tools.untargz("public_sources/gstegl_src.tbz2", self.source_folder)
        tools.rmdir("public_sources")

    def build(self):
        self.autotools(source_folder="gstegl_src/gst-egl")
        pc_path = os.path.join(self.package_folder, "lib", "pkgconfig", "gstreamer-egl-1.0.pc")
        self.run('sed -i "s/Requires: .*/Requires: gstreamer-1.0 libglvnd x11/" ' + pc_path)
        self.run('sed -i "s/Libs: .*/Libs: -L\${libdir} -lgstegl-1.0/" ' + pc_path)
        self.run('sed -i "s/Cflags: .*/Cflags: -I\${includedir}/" ' + pc_path)

from build import *

mapper = {
    "TX2": "T186",
    "Xaviar": "T186",
    "Nano": "T210",
}


class GstNvJetsonV4l2(GstRecipe):
    settings = GstRecipe.settings + ("compiler",)
    description = "NVIDIA jetson v4l2 element"
    license = "LGPL"
    options = {"jetson": ["Nano", "TX2", "Xavier"]}
    default_options = ("jetson=TX2",)
    exports_sources = {"patches/*"}
    build_requires = ("pkgconf/[^1.6.3]",)

    def requirements(self):
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")
        self.requires(f"nv-jetson-drivers/[^{self.version}]")
        self.requires(f"nv-jetson-v4l2/[^{self.version}]")

    def source(self):
        self.get(
            f"https://developer.nvidia.com/embedded/dlc/r{self.version.replace('.', '-')}_Release_v1.0/Sources/{mapper[str(self.options.jetson)]}/public_sources.tbz2"
        )
        tools.untargz(
            "Linux_for_Tegra/source/public/gst-nvvideo4linux2_src.tbz2", self.source_folder
        )
        tools.rmdir("public_sources")
        tools.patch(patch_file="patches/Makefile.patch")
        tools.patch(patch_file="patches/gstv4l2.c.patch")

    def build(self):
        env = {
            "LIB_INSTALL_DIR": os.path.join(self.deps_cpp_info["nv-jetson-drivers"].rootpath, "lib")
        }
        with tools.chdir("gst-v4l2"), tools.environment_append(env):
            self.run("make")

    def package(self):
        self.copy("*.so*", dst="lib/gstreamer-1.0", keep_path=False)

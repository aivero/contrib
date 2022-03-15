from build import *
from conans.errors import ConanInvalidConfiguration


class GstRecipe(GstRustProject):
    description = "All-batteries included GStreamer WebRTC producer"
    license = "MIT"
    exports = "*.patch"
    build_requires = ("rust/[^1.0.0]",)
    requires = ("rust-libstd/[^1.0.0]",)

    def requirements(self):
        self.requires(f"gst-plugins-bad/[~{self.settings.gstreamer}]")

    def source(self):
        self.get(f"https://github.com/centricular/webrtcsink/archive/{self.version}.tar.gz")
        self.patch("cafile.patch")
        self.patch("vp9enctuning.patch")

        project_files = os.listdir(os.path.join(self.src, "plugins"))
        for pfile in project_files:
            shutil.move(os.path.join(self.src, "plugins", pfile), self.source_folder)

        shutil.rmtree(self.src)

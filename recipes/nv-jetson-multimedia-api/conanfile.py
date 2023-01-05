from build import *


class NvJetsonMultimediaApi(Recipe):
    description = "Multimedia API is a collection of lower-level APIs that support flexible application development"
    license = "LGPL"
    exports_sources = ["public_sources.tbz2"]
    options = {"jetson": ["Nano", "TX2", "Xavier"]}
    default_options = "jetson=TX2"

    def source(self):
        if self.options.jetson in ("TX2", "Xavier"):
            self.get(
                f"https://developer.nvidia.com/embedded/dlc/r{self.version.replace('.', '-')}_Release_v1.0/TX2-AGX/Tegra_Multimedia_API_R{self.version}_aarch64.tbz2"
            )
        elif self.options.jetson == "Nano":
            self.get(
                f"https://developer.nvidia.com/embedded/dlc/r{self.version.replace('.', '-')}_Release_v1.0/Nano-TX1/Tegra_Multimedia_API_R{self.version}_aarch64.tbz2"
            )
        else:
            raise KeyError("Unknown option: " + self.options.jetson)

    def package(self):
        self.copy("*", dst="include", src="tegra_multimedia_api/include", symlinks=False)

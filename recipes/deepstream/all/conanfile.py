from conans import *


class Deepstream(ConanFile):
    description = "Complete streaming analytics toolkit for AI-based video"
    license = "proprietary"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    options = {"jetson": ["Nano", "TX2", "Xavier"]}
    default_options = ("jetson=TX2",)
    build_requires = ("generators/1.0.0",)

    def source(self):
        if self.options.jetson in ("TX2", "Xavier"):
            tools.get(f"https://developer.download.nvidia.com/assets/Deepstream/Deepstream_{self.version}/deepstream_sdk_v{self.version}_jetson.tbz2")
        else:
            raise KeyError("Unknown option: " + self.options.jetson)

        tools.untargz(f"deepstream_sdk_v{self.version}_jetson/binaries.tbz2", self.source_folder)

    def package(self):
        self.copy("*.so*", dst="lib", keep_path=False)
        self.copy(f"*", dst="include", src="deepstream_sdk_v{self.version}_jetson/sources/includes", keep_path=False)

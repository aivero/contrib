from os import path, symlink, listdir

from build import *

pc_v4lconvert = """
prefix=%s
exec_prefix=${prefix}/bin
includedir=${prefix}/include
libdir=${prefix}/lib

Name: libv4lconvert
Description: v4l2 device access library
Version: %s
Libs: -L${libdir} -lv4lconvert
Cflags: -I${includedir}
"""

pc_v4l2 = """
prefix=%s
exec_prefix=${prefix}/bin
includedir=${prefix}/include
libdir=${prefix}/lib

Name: libv4l2
Description: v4l2 device access library
Version: %s
Requires.private: libv4lconvert
Libs: -L${libdir} -lv4l2
Libs.private: -lpthread
Cflags: -I${includedir}
"""


class NvJetsonV4l2(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "NVIDIA built Accelerated GStreamer Plugins"
    license = "LGPL"
    options = {"jetson": ["Nano", "TX2", "Xavier"]}
    default_options = ("jetson=TX2",)
    exports_sources = {"patches/*"}
    build_requires = ("cc/[^1.0.0]",)

    def source(self):
        if self.version == "32.3.1":
            if self.options.jetson in ("TX2", "Xavier"):
                self.get(
                    f"https://developer.nvidia.com/embedded/dlc/r{self.version}_Release_v1.0/Sources/T186/public_sources.tbz2".replace(
                        ".", "-"
                    )
                )
            elif self.options.jetson == "Nano":
                self.get(
                    f"https://developer.nvidia.com/embedded/dlc/r{self.version}_Release_v1.0/Sources/T210/public_sources.tbz2".replace(
                        ".", "-"
                    )
                )
            else:
                raise KeyError("Unknown option: " + self.options.jetson)
            tools.untargz("Linux_for_Tegra/source/public/v4l2_libs_src.tbz2", self.source_folder)
            tools.rmdir("public_sources")
            print(listdir())
            tools.patch(patch_file="patches/Makefile.patch")
        else:
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

            tools.untargz("public_sources/v4l2_libs_src.tbz2", self.source_folder)
            tools.rmdir("public_sources")

    def build(self):
        with tools.chdir("libv4lconvert"):
            self.run("make")
            symlink("libnvv4lconvert.so", "libv4lconvert.so")
        with open("libv4lconvert.pc", "w") as pc:
            pc.write(pc_v4lconvert % (self.package_folder, self.version))

        # Make looks for libs in DEST_DIR
        env = {"DEST_DIR": path.join(self.build_folder, "libv4lconvert")}
        with tools.chdir("libv4l2"), tools.environment_append(env):
            self.run("make")
            symlink("libnvv4l2.so", "libv4l2.so")

        with open("libv4l2.pc", "w") as pc:
            pc.write(pc_v4l2 % (self.package_folder, self.version))

    def package(self):
        self.copy("*.so*", dst="lib", keep_path=False, links=True)
        self.copy("*.h", dst="include", keep_path=False)
        self.copy("*.pc", dst="lib/pkgconfig", keep_path=False)

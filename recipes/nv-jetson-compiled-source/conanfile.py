import os
import re
from build import *

from conans import AutoToolsBuildEnvironment, ConanFile, tools
from pathlib import Path

# These come from https://developer.nvidia.com/embedded/downloads

download_tx2_url_sources = {
    "32.2.1": "https://developer.nvidia.com/embedded/dlc/r32-2-1_Release_v1.0/TX2-AGX/sources/public_sources.tbz2",
    "32.3.1": "https://developer.nvidia.com/embedded/dlc/r32-3-1_Release_v1.0/Sources/T186/public_sources.tbz2",
    "32.4.3": "https://developer.nvidia.com/embedded/L4T/r32_Release_v4.3/sources/T186/public_sources.tbz2",
}
download_tx1_url_sources = {
    "32.2.1": "https://developer.nvidia.com/embedded/dlc/r32-2-1_Release_v1.0/Nano-TX1/sources/public_sources.tbz2",
    "32.3.1": "https://developer.nvidia.com/embedded/dlc/r32-3-1_Release_v1.0/Sources/T210/public_sources.tbz2",
    "32.4.3": "https://developer.nvidia.com/embedded/L4T/r32_Release_v4.3/sources/T210/public_sources.tbz2",
}


def get_lib_dir(basedir, libname):
    if "libgst" in (libname):
        return basedir + "/gstreamer-1.0"
    else:
        return basedir


class NvJetsonCompiledSource(GstProject):
    license = "LGPL"
    description = "Compiled elements from NVIDIAs Jetson sources"
    settings = {
        "os": None,
        "compiler": None,
        "build_type": None,
        "arch": "armv8",
        "hardware": {"l4t": {"board", "version"}},
        "gstreamer": None,
    }
    exports = ["*.patch"]

    build_requires = ("autotools/[^1.0.0]", "pkgconf/[^1.7.3]", "cc/[^1.0.0]")

    def requirements(self):
        self.requires(
            f"gst-plugins-base/[~{self.settings.gstreamer}]",
        )
        self.requires(f"nv-jetson-drivers/{self.settings.hardware.version}")

    def source(self):
        if self.settings.hardware.board == "t186":
            # Should also reside in Linux_for_Tegra
            tools.get(download_tx2_url_sources[f"{self.settings.hardware.version}"])
        elif self.settings.hardware.board == "t210":
            tools.get(download_tx1_url_sources[f"{self.settings.hardware.version}"])
        else:
            raise KeyError("Unknown option board type")

        # Unpack the relevant sources
        tools.untargz("Linux_for_Tegra/source/public/gstomx1_src.tbz2", self.source_folder)
        tools.patch(patch_file="default-config.patch", base_path="gstomx1_src")
        tools.patch(patch_file="configure.ac.patch", base_path="gstomx1_src")

        tools.untargz("Linux_for_Tegra/source/public/gstegl_src.tbz2", self.source_folder)
        tools.untargz(
            "Linux_for_Tegra/source/public/gst-nvvideo4linux2_src.tbz2", self.source_folder
        )
        tools.rmdir("Linux_for_Tegra")

    def build(self):

        with tools.chdir("gstegl_src/gst-egl"):
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure()
            autotools.make()
            autotools.install()
            # self.autotools()
        pc_path_base = os.path.join(self.package_folder, "lib", "pkgconfig")
        pc_path_egl = os.path.join(pc_path_base, "gstreamer-egl-1.0.pc")
        self.run('sed -i "s/Requires: .*/Requires: gstreamer-1.0 libglvnd x11/" %s' % pc_path_egl)
        self.run('sed -i "s/Libs: .*/Libs: -L\${libdir} -lgstegl-1.0/" %s' % pc_path_egl)
        self.run('sed -i "s/Cflags: .*/Cflags: -I\${includedir}/" %s' % pc_path_egl)

        env = {
            "NOCONFIGURE": "true",
            "GST_EGL_LIBS": "-lgstegl-1.0 -lnvbuf_utils -lEGL -lX11 -lgstreamer-1.0 -lgobject-2.0 -lglib-2.0",
            "PKG_CONFIG_PATH": os.environ["PKG_CONFIG_PATH"] + ":" + pc_path_base,
            "LIBRARY_PATH": os.environ["LIBRARY_PATH"]
            + ":"
            + os.path.join(self.package_folder, "lib")
            + ":"
            + os.path.join(self.build_folder, "usr/lib/aarch64-linux-gnu/tegra"),
            "CFLAGS": f" -I{self.build_folder} -Wno-error",
            "ERROR_CFLAGS": "",
        }
        args = ["--with-omx-target=tegra"]

        with tools.chdir("gstomx1_src/gst-omx1"), tools.environment_append(env):
            print(os.environ["LD_LIBRARY_PATH"])

            self.run("libtoolize --copy --force")
            self.run("aclocal -I m4 -I common/m4")
            self.run("autoheader")
            self.run("autoconf")
            self.run("automake -a -c")
            # self.run("./autogen.sh")
            autotools = AutoToolsBuildEnvironment(self)
            autotools.configure(args=args)
            autotools.make()
            autotools.install()
            # self.autotools()

    def package(self):
        lib_folder = os.path.join(self.package_folder, "lib")

        self.copy(
            "*.so*",
            src="usr",
            dst="lib",
            keep_path=False,
            symlinks=False,
            excludes=("*libgst*.so*"),
        )
        self.copy(
            "*libgst*.so*",
            dst="lib/gstreamer-1.0",
            keep_path=False,
            symlinks=False,
            excludes=("*libgstomx.*"),
        )

    def package_info(self):
        self.env_info.JETSON_DRIVER_PATH = os.path.join(self.package_folder, "lib")

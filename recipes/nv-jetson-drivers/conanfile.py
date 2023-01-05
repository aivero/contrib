from build import *
from conans import tools as conantools
import os
import re
from pathlib import Path

# These come from https://developer.nvidia.com/embedded/downloads
download_tx2_url_prebuilds = {
    "32.2.1": "https://developer.nvidia.com/embedded/dlc/r32-2-1_Release_v1.0/TX2-AGX/Tegra186_Linux_R32.2.1_aarch64.tbz2",
    "32.3.1": "https://developer.nvidia.com/embedded/dlc/r32-3-1_Release_v1.0/t186ref_release_aarch64/Tegra186_Linux_R32.3.1_aarch64.tbz2",
    "32.4.3": "https://developer.nvidia.com/embedded/L4T/r32_Release_v4.3/t186ref_release_aarch64/Tegra186_Linux_R32.4.3_aarch64.tbz2",
}
download_tx1_url_prebuilds = {
    "32.2.1": "https://developer.nvidia.com/embedded/dlc/r32-2-1_Release_v1.0/Nano-TX1/Tegra210_Linux_R32.2.1_aarch64.tbz2",
    "32.3.1": "https://developer.nvidia.com/embedded/dlc/r32-3-1_Release_v1.0/t210ref_release_aarch64/Tegra210_Linux_R32.3.1_aarch64.tbz2",
    "32.4.3": "https://developer.nvidia.com/embedded/L4T/r32_Release_v4.3/t210ref_release_aarch64/Tegra210_Linux_R32.4.3_aarch64.tbz2",
}


def get_lib_dir(basedir, libname):
    if "libgst" in (libname):
        return basedir + "/gstreamer-1.0"
    else:
        return basedir


class NvJetsonDrivers(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "NVIDIA built Accelerated GStreamer Plugins"
    license = "LGPL"
    options = {"jetson": ["Nano", "TX2", "Xavier"]}
    default_options = "jetson=TX2"

    def source(self):
        if self.settings.hardware.board == "t186":
            # Should reside in Linux_for_Tegra
            tools.get(download_tx2_url_prebuilds[f"{self.settings.hardware.version}"])
        elif self.settings.hardware.board == "t210":
            tools.get(download_tx1_url_prebuilds[f"{self.settings.hardware.version}"])
        else:
            raise KeyError("Unknown option board type")

        tools.untargz("Linux_for_Tegra/nv_tegra/nvidia_drivers.tbz2", self.source_folder)
        tools.untargz("Linux_for_Tegra/nv_tegra/nv_sample_apps/nvgstapps.tbz2", self.source_folder)

        conantools.rmdir("Linux_for_Tegra")

    def package(self):
        lib_folder = os.path.join(self.package_folder, "lib")
        list_relink = []

        path = Path(self.build_folder)
        for fp in list(path.glob("**/*.so*")):
            # Store all dedicated symlink names, then delete link. Will restore them as siblings with relative linking
            if fp.is_symlink():
                list_relink.append((fp.name, Path(os.readlink(fp)).name))
                print(
                    "Stored symlink relationship: " + fp.name + " --> " + Path(os.readlink(fp)).name
                )
                fp.unlink()
        # Now we can copy everything over including the (nonexisting) symlinks without conan creating full blown files from symlinks.
        # We exclude elements that we build manually.
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
            excludes=("*libgstomx.*", "*libgst*egl*"),
        )

        # Create symlinks .so --> .so.*
        for dl in list(Path(lib_folder).glob("**/*.so*")):
            splitname = dl.name.split(".so")
            if splitname[1] == "":
                # If we only have a .so file we don't need to symlink
                continue
            else:
                # Otherwise we need to create the file, making sure this is a sibling symlink
                with conantools.chdir(get_lib_dir(lib_folder, dl.name)):
                    nf = Path(splitname[0] + ".so")
                    nf.symlink_to(dl.name)
                    print("Created symlink: " + splitname[0] + ".so" + " --> " + dl.name)

        list_relink.append(("libv4l2.so", "libnvv4l2.so"))
        list_relink.append(("libv4l2.so.0", "libv4l2.so"))
        list_relink.append(("libv4lconvert.so", "libnvv4lconvert.so"))
        list_relink.append(("libv4lconvert.so.0", "libv4lconvert.so"))
        list_relink.append(("libcuda.so.1", "libcuda.so.1.1"))
        # list_relink.append(("libgstegl-1.0.so.0", "libgstnvegl-1.0.so.0"))
        list_relink.append(
            (
                "libnvidia-fatbinaryloader.so.440.18",
                "libnvidia-fatbinaryloader.so.%s" % self.settings.hardware.version,
            )
        )
        # Create the saved and extra symlinks, making sure to not overwrite existing ones.
        for sym, target in list_relink:
            sl = Path(sym)
            tg = Path(target)
            with tools.chdir(get_lib_dir(lib_folder, sl.name)):
                # If the new symlink to create does not exist yet, but the target does:
                if (not sl.is_file()) and tg.is_file():
                    sl.symlink_to(target)
                    print("Re-created symlink: %s --> %s " % (sl, tg))
                elif (not sl.is_file()) and (not tg.is_file()):
                    print(
                        "Couldn't re-create symlink: %s --> %s ! The target does not exist."
                        % (sl, tg)
                    )
                else:
                    print(
                        "Couldn't re-create symlink: %s --> %s ! The symlink name already exists as a file."
                        % (sl, tg)
                    )

    def package_info(self):
        self.env_info.JETSON_DRIVER_PATH = os.path.join(self.package_folder, "lib")
        self.env_info.GST_PLUGIN_PATH = os.path.join(self.package_folder, "lib/gstreamer-1.0")

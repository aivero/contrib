from build import *


class GstLibcamera(CppGstRecipe):
    description = "The libcamera package"
    license = "LGPL"
    exports = "0001-libcamerasrc-Implement-GstPhotography-interface.patch"
    options = {
        "gstreamer": ["auto", "enabled"],
        "cam": ["auto", "enabled"],
        "test": [True, False],
        "v4l2":[True, False],
        "udev": [True, False],
    }
    default_options = ("gstreamer=enabled", "cam=enabled", "test=True", "v4l2=True", "udev=True")
    build_requires = (
        "git/[^2.34.1]",
        "cc/[^1.0.0]",
        "meson/[>=0.62.0]",
        "cmake/[>=3.17]",
    )
    requires = (
        "libyaml/[>=0.1.1]",
        "python-jinja/[^3.0.0]",
        "python-pyyaml/[>=5.2]",
        "python-ply/[^3.11]",
        "gnutls/[^3.7.6]",
        "libevent/[^2.1.11]",
        "libtiff/[^4.3.0]",
        "libgudev/[^2.3.7]"
    )

    def requirements(self):
        self.requires(f"gst-plugins-bad/[~{self.settings.gstreamer}]")

    def source(self):
        self.get(f"https://github.com/kbingham/libcamera/archive/refs/tags/v{self.version}.tar.gz")
        self.patch("0001-libcamerasrc-Implement-GstPhotography-interface.patch")

    def build(self):
        req = "libtiff"
        os.environ["CXXFLAGS"] += f" -I{os.path.join(self.deps_cpp_info[req].rootpath, 'include')}"

        os.environ["CXXFLAGS"] += " -Wno-error"
        opts = {
            "gstreamer": self.options.gstreamer,
            "cam": self.options.cam,
            "test": self.options.test,
            "v4l2": self.options.v4l2,
            "udev": self.options.udev,
            "ipas": "rpi/vc4,ipu3,rkisp1,vimc"
        }
        self.meson(opts)

    def package(self):
        self.copy(pattern="*.json*", keep_path=True)
    
    def package_info(self):
        self.env_info.LIBCAMERA_DATA_DIR = os.path.join(self.package_folder, "share", "libcamera")
        self.env_info.LIBCAMERA_SYSCONF_DIR = os.path.join(self.package_folder, "etc", "libcamera")
        self.env_info.IPA_PROXY_DIR = os.path.join(self.package_folder, "bin", "libcamera")
        self.env_info.IPA_CONFIG_DIR = os.path.join(self.package_folder, "etc", "libcamera", "ipa") + ":" + os.path.join(self.package_folder, "share", "libcamera", "ipa")
        self.env_info.IPA_MODULE_DIR = os.path.join(self.package_folder, "lib", "libcamera")

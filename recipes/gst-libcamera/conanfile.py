from build import *


class GstLibcamera(GstRecipe):
    settings = GstRecipe.settings + ("compiler",)
    description = "The libcamera package"
    license = "LGPL"
    options = {
        "gstreamer": ["auto", "enabled"],
        "cam": ["auto", "enabled"],
        "test": [True, False],
    }
    default_options = (
        "gstreamer=enabled",
        "cam=enabled",
        "test=True"
    )
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
    )
    def requirements(self):
        self.requires(f"gst-plugins-bad/[~{self.settings.gstreamer}]")

    def source(self):
        self.get(f"https://github.com/kbingham/libcamera/archive/refs/tags/v{self.version}.tar.gz")

    def build(self):
        req = "libtiff"
        os.environ[
            "CXXFLAGS"
        ] += f" -I{os.path.join(self.deps_cpp_info[req].rootpath, 'include')}"

        os.environ["CXXFLAGS"] += " -Wno-error"
        opts = {
            "gstreamer": self.options.gstreamer,
            "cam": self.options.cam,
            "test": self.options.test,
        }
        self.meson(opts)


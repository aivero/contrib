from build import *


class LibNiceRecipe(GstRecipe):
    description = (
        "An implementation of the IETF's Interactive Connectivity Establishment (ICE) standard"
    )
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
    )
    requires = ("openssl1/[>=1.1.1h]",)

    def requirements(self):
        # This will SemVer match PATH changes, but not MINOR or MAJOR changes
        # That way we can still build for a lower gst minor release (i.e. 1.18), despite a newer one being in your conan (i.e. 1.19)
        # [^1.18] will match any `1.` version - not what we need
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")

    exports_sources = "nice.pc"

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/libnice/libnice/-/archive/{self.version}/libnice-{self.version}.tar.gz"
        )

    def package(self):
        tools.replace_prefix_in_pc_file("nice.pc", self.package_folder)
        self.copy("nice.pc", dst="lib/pkgconfig")

    def build(self):
        opts = {
            "gstreamer": True,
        }
        self.meson(opts)

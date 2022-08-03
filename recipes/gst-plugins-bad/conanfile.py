from build import *
from conans.errors import ConanInvalidConfiguration


class GstPluginsBad(GstRecipe):
    description = "A set of plugins that aren't up to par compared to the rest"
    license = "LGPL"
    exports = "*.patch"
    options = {
        "closedcaption": [True, False],
        "debugutils": [True, False],
        "dtls": [True, False],
        "gl": [True, False],
        "inter": [True, False],
        "introspection": [True, False],
        "mpegtsdemux": [True, False],
        "mpegtsmux": [True, False],
        "msdk": [True, False],
        "nvcodec": [True, False],
        "opencv": [True, False],
        "pnm": [True, False],
        "rtmp2": [True, False],
        "srtp": [True, False],
        "sctp": [True, False],
        "videoparsers": [True, False],
        "webp": [True, False],
        "webrtc": [True, False],
        "x265": [True, False],
        "onvif": [True, False],
    }
    default_options = (
        "closedcaption=False",
        "debugutils=True",
        "dtls=True",
        "gl=True",
        "inter=False",
        "introspection=True",
        "mpegtsdemux=True",
        "mpegtsmux=True",
        "msdk=False",
        "nvcodec=True",
        "opencv=False",
        "pnm=True",
        "rtmp2=True",
        "srtp=True",
        "sctp=True",
        "videoparsers=True",
        "webp=True",
        "webrtc=True",
        "x265=False",
        "onvif=True",
    )

    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "gobject-introspection/[^1.69.0]",
    )
    requires = ("libnice/[^0.1.18]",)

    def configure(self):
        if self.settings.arch != "x86_64":
            self.options.remove("nvcodec")
            self.options.remove("msdk")

    def validate(self):
        if str(self.settings.gstreamer) not in str(self.version):
            raise ConanInvalidConfiguration(
                f"GStreamer version specified in devops.yml ({self.version}) is not compatible with version specified in profile: {self.settings.gstreamer}"
            )

    def build_requirements(self):
        #     # This will SemVer match PATH changes, but not MINOR or MAJOR changes
        #     # That way we can still build for a lower gst minor release (i.e. 1.18), despite a newer one being in your conan (i.e. 1.19)
        #     # [^1.18] will match any `1.` version - not what we need
        #     # We only build_requires base because we already transitivly requires it through libnice.
        #     self.build_requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")

        if self.settings.arch == "x86_64" and self.options.nvcodec:
            self.build_requires("cuda/[>=11.2]")
            self.build_requires("orc/[^0.4.31]")

    def requirements(self):
        if self.options.srtp:
            self.requires("libsrtp/[^2.2.0]")
        if self.options.opencv:
            self.requires("opencv/[^3.4.8]")
        if self.options.closedcaption:
            self.requires("pango/[^1.4.3]")
        if self.options.webp:
            self.requires("libwebp/[^1.1.0]")
        if self.settings.arch == "x86_64" and (self.options.msdk):
            self.requires("intel-media-sdk/[>=20.2]")
            self.requires("libgudev/[>=233]")
        if self.options.x265:
            self.requires("x265/[>=2.7]")

    def source(self):
        if "1.21" in self.version:
            # until the changes from https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2132 and https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2432 are tagged we need to use a commit of the main branch
            self.get(
                f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/ab459f0528d8eba17589c8a8582b69a166616384.tar.gz"
            )
        else:
            self.get(
                f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/{self.version}.tar.gz"
            )

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-plugins-bad")
        opts = {
            "closedcaption": self.options.closedcaption,
            "debugutils": self.options.debugutils,
            "dtls": self.options.dtls,
            "gl": self.options.gl,
            "inter": self.options.inter,
            "mpegtsdemux": self.options.mpegtsdemux,
            "mpegtsmux": self.options.mpegtsmux,
            "opencv": self.options.opencv,
            "pnm": self.options.pnm,
            "rtmp2": self.options.rtmp2,
            "srtp": self.options.srtp,
            "sctp": self.options.sctp,
            "videoparsers": self.options.videoparsers,
            "webp": self.options.webp,
            "webrtc": self.options.webrtc,
            "x265": self.options.x265,
            "onvif": self.options.onvif,
        }
        if self.settings.arch == "x86_64":
            opts["msdk"] = self.options.msdk
            opts["nvcodec"] = self.options.nvcodec

        if self.options.x265:
            self.license = "GPL"
            opts["gpl"] = "enabled"

        self.meson(opts, source_folder)

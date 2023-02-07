from build import *
from conans.errors import ConanInvalidConfiguration


class GstPluginsGood(GstRecipe):
    description = "Plug-ins is a set of plugins that we consider to have good quality code and correct functionality"
    license = "LGPL"
    # If set to true, select version highest semver matching version from devops.yml
    gst_match_version = True
    settings = GstRecipe.settings + ("hardware",)
    exports = "*.patch"
    options = {
        "aivero_rvl_matroska": [True, False],
        "autodetect": [True, False],
        "isomp4": [True, False],
        "jpeg": [True, False],
        "matroska": [True, False],
        "multifile": [True, False],
        "png": [True, False],
        "rtp": [True, False],
        "rtsp": [True, False],
        "udp": [True, False],
        "videofilter": [True, False],
        "videomixer": [True, False],
        "vpx": [True, False],
        "ximagesrc_xdamage": [True, False],
        "ximagesrc_xshm": [True, False],
        "ximagesrc": [True, False],
        "v4l2": [True, False],
    }
    default_options = (
        "aivero_rvl_matroska=True",
        "autodetect=True",
        "isomp4=True",
        "jpeg=True",
        "matroska=True",
        "multifile=True",
        "png=True",
        "rtp=True",
        "rtsp=True",
        "udp=True",
        "videofilter=True",
        "videomixer=True",
        "vpx=True",
        "ximagesrc_xdamage=False",
        "ximagesrc_xshm=True",
        "ximagesrc=True",
        "v4l2=True",
    )

    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.62.0]",
        "git/[^2.30.0]",
    )

    def validate(self):
        if str(self.settings.gstreamer) not in str(self.version):
            raise ConanInvalidConfiguration(
                f"GStreamer version specified in devops.yml ({self.version}) is not compatible with version specified in profile: {self.settings.gstreamer}"
            )

    def configure(self):
        if self.settings.hardware == "rpi":
            if self.settings.arch == "armv8":
                # enable v4l2 for rpi 64-bit
                self.options.v4l2 = True

    def requirements(self):
        # This will SemVer match PATH changes, but not MINOR or MAJOR changes
        # That way we can still build for a lower gst minor release (i.e. 1.18), despite a newer one being in your conan (i.e. 1.19)
        # [^1.18] will match any `1.` version - not what we need
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")

        self.requires(f"libgudev/[^2.3.7]")

        # gst-plugins-bad -> pango -> freetype -> png
        # if self.options.png:
        #     self.requires("libpng/[^1.6.37]")
        if self.options.vpx:
            self.requires("libvpx/[^1.8.0]")
        if self.options.jpeg:
            self.requires("libjpeg-turbo/[^2.0.3]")

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/{self.version}.tar.gz"
        )
        # Add our own custom changes
        self.patch("0001-matroska-Support-any-tag-1.22.0.patch")
        self.patch("0001-v4l2deviceprovider-Return-device-even-if-caps-is-EMP.patch")
        if self.options.aivero_rvl_matroska:
            self.patch("0002-matroska-add-support-for-custom-video-rvl-1.22.0.patch")

        self.patch("0003-v4l2src-video-bitrate-control-1.22.0.patch")

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-plugins-good")
        opts = {
            "autodetect": True,
            "isomp4": True,
            "jpeg": True,
            "matroska": True,
            "multifile": True,
            "png": True,
            "rtp": True,
            "rtpmanager": True,
            "rtsp": True,
            "udp": True,
            "videofilter": True,
            "videomixer": True,
            "vpx": True,
            "ximagesrc-xdamage": True,
            "ximagesrc": True,
            "debugutils": True,
            "audiofx": True,
            "v4l2": self.options.v4l2,
            "v4l2-gudev": self.options.v4l2,
        }
        self.meson(opts, source_folder)

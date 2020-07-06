import os
from conans import *


class GStreamerPluginsBadConan(ConanFile):
    description = "A set of plugins that aren't up to par compared to the rest"
    license = "LGPL"
    exports = "reduce_latency.patch"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    options = {
        "introspection": [True, False],
        "videoparsers": [True, False],
        "gl": [True, False],
        "nvdec": [True, False],
        "nvenc": [True, False],
        "nvcodec": [True, False],
        "pnm": [True, False],
        "webrtc": [True, False],
        "srtp": [True, False],
        "rtmp2": [True, False],
        "dtls": [True, False],
        "mpegtsmux": [True, False],
        "mpegtsdemux": [True, False],
        "debugutils": [True, False],
        "opencv": [True, False],
        "closedcaption": [True, False],
        "aiveropatchlatency": [True, False],
        "inter": [True, False],
        "webp": [True, False],
    }
    default_options = (
        "introspection=True",
        "videoparsers=True",
        "gl=True",
        "nvdec=False",
        "nvenc=False",
        "nvcodec=False",
        "pnm=True",
        "webrtc=True",
        "srtp=True",
        "rtmp2=True",
        "dtls=True",
        "mpegtsmux=True",
        "mpegtsdemux=True",
        "debugutils=True",
        "opencv=False",
        "closedcaption=False",
        "aiveropatchlatency=False",
        "inter=False",
        "webp=True",
    )
    build_requires = (
        "base/[^1.0.0]",
        "meson/[^0.51.2]",
        "gobject-introspection/[^1.59.3]",
    )
    requires = (
        "glib/[^2.62.0]",
        "gstreamer-plugins-base/[~1.16]",
    )

    def configure(self):
        if self.settings.arch != "x86_64":
            self.options.remove("nvdec")
            self.options.remove("nvenc")
            self.options.remove("nvcodec")

    def build_requirements(self):
        if self.settings.arch == "x86_64" and (self.options.nvenc or self.options.nvdec):
            self.build_requires("cuda/[~10.1]")
            self.build_requires("orc/[^0.4.31]")

    def requirements(self):
        if self.options.webrtc:
            self.requires("libnice/[^0.1]")
        if self.options.srtp:
            self.requires("libsrtp/[^2.2.0]")
        if self.options.opencv:
            self.requires("opencv/[^3.4.8]")
        if self.options.closedcaption:
            self.requires("pango/[^1.4.3]")
        if self.options.webp:
            self.requires("libwebp/[^1.1.0]")

    def source(self):
        git = tools.Git(folder="gst-plugins-bad-" + self.version)
        git.clone(url="https://gitlab.freedesktop.org/gstreamer/gst-plugins-bad.git", branch=self.version, shallow=True)
        if self.options.aiveropatchlatency:
            tools.patch(patch_file="reduce_latency.patch", base_path=os.path.join(self.source_folder, "gst-plugins-bad"))

    def build(self):
        args = ["--auto-features=disabled"]
        args.append("-Dvideoparsers=" + ("enabled" if self.options.videoparsers else "disabled"))
        args.append("-Dgl=" + ("enabled" if self.options.gl else "disabled"))
        args.append("-Dpnm=" + ("enabled" if self.options.pnm else "disabled"))
        args.append("-Dwebrtc=" + ("enabled" if self.options.webrtc else "disabled"))
        args.append("-Dsrtp=" + ("enabled" if self.options.srtp else "disabled"))
        args.append("-Drtmp2=" + ("enabled" if self.options.rtmp2 else "disabled"))
        args.append("-Ddtls=" + ("enabled" if self.options.srtp else "disabled"))
        args.append("-Dmpegtsmux=" + ("enabled" if self.options.mpegtsmux else "disabled"))
        args.append("-Dmpegtsdemux=" + ("enabled" if self.options.mpegtsdemux else "disabled"))
        args.append("-Ddebugutils=" + ("enabled" if self.options.debugutils else "disabled"))
        args.append("-Dopencv=" + ("enabled" if self.options.opencv else "disabled"))
        args.append("-Dclosedcaption=" + ("enabled" if self.options.closedcaption else "disabled"))
        args.append("-Dinter=" + ("enabled" if self.options.inter else "disabled"))
        args.append("-Dwebp=" + ("enabled" if self.options.webp else "disabled"))
        if self.settings.arch == "x86_64":
            args.append("-Dnvdec=" + ("enabled" if self.options.nvdec else "disabled"))
            args.append("-Dnvenc=" + ("enabled" if self.options.nvenc else "disabled"))
            if self.version == "master":
                args.append("-Dnvcodec=" + ("enabled" if self.options.nvcodec else "disabled"))

        meson = Meson(self)
        meson.configure(source_folder=f"gst-plugins-bad-{self.version}", args=args, pkg_config_paths=os.environ["PKG_CONFIG_PATH"].split(":"))
        meson.build()
        meson.install()

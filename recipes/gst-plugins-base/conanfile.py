from build import *
from conans.errors import ConanInvalidConfiguration


class GstPluginsBase(GstRecipe):
    description = "A well-groomed and well-maintained collection of GStreamer plugins and elements"
    license = "LGPL"
    exports = "*.patch"
    options = {
        "shared": [True, False],
        "introspection": [True, False],
        "x11": [True, False],
        "audioresample": [True, False],
        "pango": [True, False],
        "opus": [True, False],
    }
    default_options = (
        "shared=True",
        "introspection=True",
        "x11=True",
        "audioresample=True",
        "pango=True",
        "opus=False",
    )
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.62.0]",
    )
    requires = (
        "orc/[^0.4.34]",
        "mesa/[>=20.2.1]",
    )

    def requirements(self):
        # This will SemVer match PATH changes, but not MINOR or MAJOR changes
        # That way we can still build for a lower gst minor release (i.e. 1.18), despite a newer one being in your conan (i.e. 1.19)
        # [^1.18] will match any `1.` version - not what we need
        self.requires(f"gst/[~{self.settings.gstreamer}]")
        if self.options.pango:
            self.requires("pango/[^1.43.0]")
        if self.options.opus:
            self.requires("opus/[^1.3.1]")


    def validate(self):
        if str(self.settings.gstreamer) not in str(self.version):
            raise ConanInvalidConfiguration(
                f"GStreamer version specified in devops.yml ({self.version}) is not compatible with version specified in profile: {self.settings.gstreamer}"
            )

    def build_requirements(self):
        if self.options.introspection:
            self.build_requires("gobject-introspection/[^1.66.1]")

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/{self.version}.tar.gz"
        )
        self.patch("0006-rtpbasedepayload-fix-hdrext-handling-for-aggregated-out-buffer.patch")

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-plugins-base")
        opts = {
            "gl_platform": "egl",
            "introspection": self.options.introspection,
            "x11": self.options.x11,
            "audioresample": self.options.audioresample,
            "gl": True,
            "videotestsrc": True,
            "audiotestsrc": True,
            "app": True,
            "playback": True,
            "typefind": True,
            "orc": True,
            "opus": self.options.opus,
            "pango": self.options.pango,
            "audioconvert": True,
            "compositor": True,
            "encoding": True,
            "audiomixer": True,
            "videorate": True,
            "tools": True,
            "videoconvertscale": True
        }

        self.meson(opts, source_folder)

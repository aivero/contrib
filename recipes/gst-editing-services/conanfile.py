from build import *


class GstEditingServices(GstRecipe):
    description = " This is a high-level library for facilitating the creation of audio/video non-linear editors."
    license = "LGPL"
    exports = "*.patch"
    options = {
        "doc": [True, False],
        "examples": [True, False],
        "introspection": [True, False],
        "tests": [True, False],
        "tools": [True, False],
        "bash_completion": [True, False],
        "pygi_overrides_dir": [True, False],
        "xptv": [True, False],
        "python": [True, False],
        "libpython_dir": [True, False],
        "validate": [True, False],
    }
    default_options = (
        "doc=False",
        "examples=False",
        "introspection=True",
        "tests=False",
        "tools=True",
        "bash_completion=False",
        "pygi_overrides_dir=False",
        "xptv=False",
        "python=False",
        "libpython_dir=False",
        "validate=False",
    )
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "flex/[^2.6.4]",
        "m4/[^1.4.18]",
    )

    def requirements(self):
        # This will SemVer match PATH changes, but not MINOR or MAJOR changes
        # That way we can still build for a lower gst minor release (i.e. 1.18), despite a newer one being in your conan (i.e. 1.19)
        # [^1.18] will match any `1.` version - not what we need
        self.requires(f"gst-plugins-base/[~{self.settings.gstreamer}]")

    def build_requirements(self):
        if self.options.introspection:
            self.build_requires("gobject-introspection/[^1.66.1]")

    def source(self):
        if "1.21" in self.version:
            # until the changes from https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2132 and https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/2432 are tagged we need to use a commit of the main branch
            self.get(f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/ab459f0528d8eba17589c8a8582b69a166616384.tar.gz")
        else:           
            self.get(f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/{self.version}.tar.gz")

        self.patch("ges_launch_custom_config.patch")

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gst-editing-services")
        opts = {
            "doc": self.options.doc,
            "examples": self.options.examples,
            "introspection": self.options.introspection,
            "tests": self.options.tests,
            "tools": self.options.tools,
            "bash-completion": self.options.bash_completion,
            "pygi-overrides-dir": self.options.pygi_overrides_dir,
            "xptv": self.options.xptv,
            "python": self.options.python,
            "libpython-dir": self.options.libpython_dir,
            # TODO: take this from options!!!
            "validate": False,
        }
        self.meson(opts, source_folder)

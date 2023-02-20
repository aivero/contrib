from build import *
from conans.errors import ConanInvalidConfiguration


class Gst(GstRecipe):
    description = "A framework for streaming media"
    # If set to true, select version highest semver matching version from devops.yml
    gst_match_version = True

    license = "LGPL"
    options = {
        "shared": [True, False],
        "introspection": [True, False],
    }
    default_options = (
        "shared=True",
        "introspection=True",
    )
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.62.0]",
        "bison/[^3.7.2]",
        "flex/[^2.6.4]",
    )
    requires = ("glib/[^2.70.3]",)

    def validate(self):
        if str(self.settings.gstreamer) not in str(self.version):
            raise ConanInvalidConfiguration(
                f"GStreamer version specified in devops.yml ({self.version}) is not compatible with version specified in profile: {self.settings.gstreamer}"
            )

    def build_requirements(self):
        if self.options.introspection:
            self.build_requires(
                "gobject-introspection/[^1.69.0]",
            )

    def source(self):
        self.get(
            f"https://gitlab.freedesktop.org/gstreamer/gstreamer/-/archive/47c183cdfdd75fc0baf8218aae6621df1fe4e87b.tar.gz"
        )

    def build(self):
        source_folder = os.path.join(self.src, "subprojects", "gstreamer")
        opts = {
            "check": True,
            "tools": True,
            "introspection": self.options.introspection,
        }
        self.meson(opts, source_folder)

    def package_info(self):
        self.env_info.GST_PLUGIN_SCANNER = os.path.join(
            self.package_folder, "bin", "gstreamer-1.0", "gst-plugin-scanner"
        )

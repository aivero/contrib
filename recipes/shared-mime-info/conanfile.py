from build import *


class SharedMimeInfo(Recipe):
    description = "Freedesktop.org Shared MIME Info"
    license = "GPL"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "itstool/[^2.0.6]",
        "xz/[^5.2.4]",
        "gettext/[^0.21]",
        "xmlto/[^0.0.28]",
    )
    requires = (
        "glib/[^2.70.3]",
        "libxml2/[^2.9.9]",
    )

    def source(self):
        # Fixes build with new meson
        # TODO: upgrade to new stable release with fix
        version = "3c064c850488ede673cc87dfd5da29d36384b8f4"
        self.get(
            f"https://gitlab.freedesktop.org/xdg/shared-mime-info/-/archive/{version}/shared-mime-info-{version}.tar.gz"
        )
        tools.replace_in_file(
            os.path.join(self.src, "data", "meson.build"),
            "build_by_default: true",
            "build_by_default: false",
        )

    def build(self):
        opts = {
            "update-mimedb": True,
        }
        self.meson(opts)

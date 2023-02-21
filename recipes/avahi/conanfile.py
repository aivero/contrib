from build import *

class Avahi(Recipe):
    description = "Avahi - Service Discovery for Linux using mDNS/DNS-SD -- compatible with Bonjour"
    license = "GPL"
    settings = Recipe.settings + ("python",)
    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")
    requires = (
        "glib/[^2.70.0]",
        "libdaemon/[^0.14]",
        "dbus/[^1.12.20]",
        "gdbm/[^1.19]",
        "libevent/[^2.1.11]",
        "six/[^1.16.0]",
    )

    def requires(self):
       self.requires(f"python/[^{self.settings.python}]")


    def source(self):
        self.get(f"https://github.com/lathiat/avahi/releases/download/v{self.version}/avahi-{self.version}.tar.gz")

    def build(self):
        # We set this variable to allow bindgen to look for extra include folders in the avahi package
        os.environ["BINDGEN_EXTRA_CLANG_ARGS"] = f'-I"{os.path.join(self.package_folder, "include")}"'

        args = [
            "--enable-compat-libdns_sd",
            "--disable-gtk3",
            "--disable-mono",
            "--disable-monodoc",
            "--disable-python",
            "--disable-qt5",
            "--disable-console",
            "--disable-manpages",
        ]
        self.autotools(args)
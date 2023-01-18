from build import *

from conan import ConanFile
from conan.tools.env import VirtualBuildEnv
from conan.tools.files import copy, get, rmdir, rm
from conan.tools.gnu import Autotools, AutotoolsDeps, AutotoolsToolchain, PkgConfigDeps
from conan.tools.layout import basic_layout
from conan.errors import ConanInvalidConfiguration

required_conan_version = ">=1.53.0"

class Avahi(Recipe):
    name = "avahi"
    description = "Avahi - Service Discovery for Linux using mDNS/DNS-SD -- compatible with Bonjour"
    license = "GPL"

    options = {
        "shared": [True, False],
        "fPIC": [True, False]
    }
    default_options = {
        "shared": False,
        "fPIC": True
    }

    def validate(self):
        if self.info.settings.os != "Linux":
            raise ConanInvalidConfiguration(f"{self.ref} only supports Linux.")
    
    def configure(self):
        if self.options.shared:
            self.options.rm_safe("fPIC")
        self.settings.rm_safe("compiler.cppstd")
        self.settings.rm_safe("compiler.libcxx")

    build_requires = ("cc/[^1.0.0]", "autotools/[^1.0.0]")
    requires = (
        "glib/[^2.70.0]",
        "libdaemon/[^0.14]",
        "dbus/[^1.12.20]",
        "gdbm/[^1.19]",
        "libevent/[^2.1.11]",
        "python/[^3.8.12]",
        "six/[^1.16.0]",
    )


    def source(self):
        self.get(f"https://github.com/lathiat/avahi/releases/download/v{self.version}/avahi-{self.version}.tar.gz")


    def build(self):
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
    
    def package(self):
        copy(self, "LICENSE", self.source_folder, os.path.join(self.package_folder, "licenses"))

    def package_info(self):
        for lib in ("client", "common", "core", "glib", "gobject", "libevent", "compat-libdns_sd"):
            avahi_lib = f"avahi-{lib}"
            self.cpp_info.components[lib].names["cmake_find_package"] = lib
            self.cpp_info.components[lib].names["cmake_find_package_multi"] = lib
            self.cpp_info.components[lib].names["pkg_config"] = avahi_lib
            self.cpp_info.components[lib].libs = [avahi_lib]
            self.cpp_info.components[lib].includedirs = [os.path.join("include", avahi_lib)]
        self.cpp_info.components["compat-libdns_sd"].libs = ["dns_sd"]

        self.cpp_info.components["client"].requires = ["common", "dbus::dbus"]
        self.cpp_info.components["common"].system_libs = ["pthread"]
        self.cpp_info.components["core"].requires = ["common"]
        self.cpp_info.components["glib"].requires = ["common", "glib::glib"]
        self.cpp_info.components["gobject"].requires = ["client", "glib"]
        self.cpp_info.components["libevent"].requires = ["common", "libevent::libevent"]
        self.cpp_info.components["compat-libdns_sd"].requires = ["client"]
        self.cpp_info.components["__"].requires = ["python::python", "six::six"]


        for app in ("autoipd", "browse", "daemon", "dnsconfd", "publish", "resolve", "set-host-name"):
            avahi_app = f"avahi-{app}"
            self.cpp_info.components[app].names["cmake_find_package"] = app
            self.cpp_info.components[app].names["cmake_find_package_multi"] = app
            self.cpp_info.components[app].names["pkg_config"] = avahi_app

        self.cpp_info.components["autoipd"].requires = ["libdaemon::libdaemon"]
        self.cpp_info.components["browse"].requires = ["client", "gdbm::gdbm"]
        self.cpp_info.components["daemon"].requires = ["core", "dbus::expat", "libdaemon::libdaemon"]
        self.cpp_info.components["dnsconfd"].requires = ["common", "libdaemon::libdaemon"]
        self.cpp_info.components["publish"].requires = ["client"]
        self.cpp_info.components["resolve"].requires = ["client"]
        self.cpp_info.components["set-host-name"].requires = ["client"]

        bin_path = os.path.join(self.package_folder, "bin")
        self.output.info(f"Appending PATH environment variable: {bin_path}")
        self.env_info.PATH.append(bin_path)
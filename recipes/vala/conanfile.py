from build import *


class Vala(Recipe):
    description = "Compiler for the GObject type system"
    license = "LGPL"
    build_requires = (
        "libxslt/[^1.1.34]",
        "git/[^2.34.1]",
    )
    requires = (
        "autotools/[^1.0.0]",
        "autoconf-archive/[>=2019.01.06]",
        "gobject-introspection/[^1.70.0]",
        "cc/[^1.0.0]",
        "graphviz/[^2.42.1]",
    )

    def source(self):
        git = tools.Git(folder=self.src)
        git.clone("https://gitlab.gnome.org/GNOME/vala.git", self.version)

    def build(self):
        os.environ["CPATH"] += ":" + self.deps_cpp_info["graphviz"].include_paths[0]
        self.exe(f"sh autogen.sh --prefix={self.package_folder}")
        self.exe("make")
        self.exe("make install")

    def package_info(self):
        # TODO: hardcoded version
        self.env_info.LD_LIBRARY_PATH += [
            os.path.join(self.package_folder, "lib"),
            os.path.join(self.package_folder, "lib", "vala-0.56"),
        ]

from build import *


class Xrdesktop(Recipe):
    description = "A library for XR interaction with classical desktop compositors"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "python-gobject/[^3.33.1]",
        "git/[^2.34.1]",
    )
    requires = ("g3k/[^0.16.0]",)

    def source(self):
        git = tools.Git(folder=f"{self.name}-{self.version}.src")
        git.clone("https://gitlab.freedesktop.org/xrdesktop/xrdesktop.git", self.version)


    def package(self):
        self.copy(pattern="*/shell", dst=os.path.join(self.package_folder, "bin"), keep_path=False)
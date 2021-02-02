from build import *


class GoRecipe(Recipe):
    description = "Core compiler tools for the Go programming language"
    license = "BSD"

    def source(self):
        arch = {"x86_64": "amd64", "armv8": "arm64"}[str(self.settings.arch_build)]
        filename = f"go{self.version}.linux-{arch}.tar.gz"
        tools.download(f"https://dl.google.com/go/{filename}", filename)
        # Workaround: Python3 in Ubuntu 18.04 does not support ascii encoded tarballs
        self.run("tar -xf " + filename)

    def package(self):
        self.copy("*", src="go/bin", dst="bin")
        self.copy("*", src="go/src", dst="src")
        self.copy("*", src="go/pkg", dst="pkg")

    def package_info(self):
        self.env_info.GOROOT = self.package_folder
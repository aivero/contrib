from build import *


class Go(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Core compiler tools for the Go programming language"
    license = "BSD"

    def source(self):
        arch = {"x86_64": "amd64", "armv8": "arm64"}[str(self.settings.arch)]
        filename = f"go{self.version}.linux-{arch}.tar.gz"
        tools.download(f"https://dl.google.com/go/{filename}", filename)
        # Workaround: Python3 in Ubuntu 18.04 does not support ascii encoded tarballs
        self.run(f"tar -xf {filename}")

    def package(self):
        self.copy("go/*")
        bin_path = os.path.join(self.package_folder, "bin")
        os.mkdir(bin_path)
        os.symlink("../go/bin/go", os.path.join(bin_path, "go"))
        os.symlink("../go/bin/gofmt", os.path.join(bin_path, "gofmt"))

    def package_info(self):
        self.env_info.GOROOT = os.path.join(self.package_folder, "go")

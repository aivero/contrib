from build import *


class LinuxHeaders(Recipe):
    description = "Linux headers"
    license = "GPL"
    options = {}
    default_options = {}
    settings = "os", "arch"

    def source(self):
        self.get(f"https://cdn.kernel.org/pub/linux/kernel/v{self.version.split('.')[0]}.x/linux-{self.version}.tar.xz")

    def build(self):
        arch = {"x86_64": "x86_64", "armv8": "arm64"}[str(self.settings.arch)]
        args = [
            f"ARCH={arch}",
            f'INSTALL_HDR_PATH="{self.package_folder}"',
        ]
        self.make(args, target="headers_install")

    def package_info(self):
        self.env_info.CPATH.append(os.path.join(self.package_folder, "include"))

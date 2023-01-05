from build import *


class ZigBootstrap(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "a general-purpose programming language and toolchain for maintaining robust, optimal, and reusable software"
    license = "MIT"

    def build(self):
        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch)]
        self.get(
            f"https://ziglang.org/download/{self.version}/zig-linux-{arch}-{self.version}.tar.xz"
        )

    def package(self):
        os.mkdir(os.path.join(self.package_folder, "bin"))
        for exe in ("cc", "c++", "ar", "ranlib", "ld.lld"):
            exe_path = os.path.join(self.package_folder, "bin", exe)
            with open(exe_path, "w") as exe_file:
                exe_file.write("#!/usr/bin/env sh\n")
                exe_file.write(f'zig {exe} "$@"')
            os.chmod(exe_path, 0o775)
        self.copy("lib/*", src=self.src)
        self.copy("zig", src=self.src, dst="bin")

    def package_info(self):
        self.env_info.CC = os.path.join(self.package_folder, "bin", "cc")
        self.env_info.CXX = os.path.join(self.package_folder, "bin", "c++")
        self.env_info.AS = os.path.join(self.package_folder, "bin", "cc")
        self.env_info.AR = os.path.join(self.package_folder, "bin", "ar")
        self.env_info.RANLIB = os.path.join(self.package_folder, "bin", "ranlib")
        self.env_info.LD = os.path.join(self.package_folder, "bin", "ld.lld")

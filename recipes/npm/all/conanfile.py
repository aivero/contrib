import os, shutil

from conans import *


class NpmConan(ConanFile):
    name = "npm"
    description = "Evented I/O for V8 javascript"
    license = "MIT"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    build_requires = (
        "autotools/1.0.0",
        "python/[^3.7.4]",
        "libpng/[^1.6.37]",
        "mozjpeg/[^3.3.1]",
        "pngquant/[^2.12.6]",
    )
    requires = (
        "base/[^1.0.0]",
        "nodejs/[^13.0.1]",
    )

    def source(self):
        tools.get(f"https://github.com/npm/cli/archive/v{self.version}.tar.gz")

    def build(self):
        pngquant_src = os.path.join(self.deps_cpp_info["pngquant"].rootpath, "bin", "pngquant")
        pngquant_dir = os.path.join(f"cli-{self.version}", "docs", "node_modules", "pngquant-bin", "vendor")
        os.makedirs(pngquant_dir)
        pngquant_dst = os.path.join(pngquant_dir, "pngquant")
        shutil.copy2(pngquant_src, pngquant_dst)
        with tools.chdir(f"cli-{self.version}"):
            autotools = AutoToolsBuildEnvironment(self)
            self.run("mkdir -p man/man1")
            autotools.install(['NPMOPTS=--prefix="%s"' % self.package_folder])

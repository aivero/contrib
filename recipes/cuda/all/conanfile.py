import os

from conans import *

driver_map = {"10.1.243": "418.87.00"}


class CudaConan(ConanFile):
    description = "NVIDIA's GPU programming toolkit"
    license = "custom"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    exports_sources = ("cuda-10.1.pc", "cudart-10.1.pc")
    build_requires = (
        "gcc/7.4.0",
        "libxml2/[^2.9.10]",
    )

    def source(self):
        tools.download(f"http://developer.download.nvidia.com/compute/cuda/10.1/Prod/local_installers/cuda_{self.version}_{driver_map[self.version]}_linux.run", filename=f"cuda_{self.version}_linux.run")

    def build(self):
        self.run(f'sh cuda_{self.version}_linux.run --silent --override-driver-check --extract="{self.build_folder}"')
        os.remove(f"cuda_{self.version}_linux.run")
        self.run(f"sh NVIDIA-Linux-x86_64-{driver_map[self.version]}.run --extract-only")
        os.remove(f"NVIDIA-Linux-x86_64-{driver_map[self.version]}.run")
        tools.rmdir("cublas")
        tools.rmdir("cuba-samples")

    def package(self):
        for toolkit in ("cuda-toolkit", "cuda-toolkit/nvvm"):
            self.copy("*", dst="bin", src=f"{toolkit}/bin")
            self.copy("*", dst="lib", src=f"{toolkit}/lib64")
            self.copy("*", dst="include", src=f"{toolkit}/include")
        self.copy("*.bc", src="cuda-toolkit")
        self.copy("*libcuda.so*", dst="lib", keep_path=False, symlinks=True)
        self.copy("*libnvcuvid.so*", dst="lib", keep_path=False, symlinks=True)
        with tools.chdir(os.path.join(self.package_folder, "lib")):
            os.symlink("libnvcuvid.so.418.87.00", "libnvcuvid.so.1")
            os.symlink("libnvcuvid.so.1", "libnvcuvid.so")
        self.copy(pattern="*.pc", dst="lib/pkgconfig")

from build import *

driver_map = {
    "10.1.243": "418.87.00",
    "11.1.0": "455.23",
    "11.2.1": "460.32.03",
}

arch_map = {
    "armv8": "sbsa",
    "x86_64": "x86_64",
}


nv_arch_map = {
    "armv8": "aarch64",
    "x86_64": "x86_64",
}


class CudaRecipe(CppPythonRecipe):
    description = "NVIDIA's GPU programming toolkit"
    license = "Proprietary"
    exports_sources = "*.pc"
    build_requires = (
        "cc/[^1.0.0]",
        "libxml2/[^2.9.10]",
    )

    def build(self):
        arch = arch_map[str(self.settings.arch)]
        nv_arch = nv_arch_map[str(self.settings.arch)]
        nv_version = driver_map[self.version]
        if self.settings.arch == "x86_64":
            if self.version.startswith("10"):
                version_short = ".".join(self.version.split(".")[:2])
                tools.download(
                    f"https://developer.download.nvidia.com/compute/cuda/{version_short}/Prod/local_installers/cuda_{self.version}_{nv_version}_linux.run",
                    filename=f"cuda.run",
                )
            else:
                tools.download(
                    f"https://developer.download.nvidia.com/compute/cuda/{self.version}/local_installers/cuda_{self.version}_{nv_version}_linux.run",
                    filename=f"cuda.run",
                )
        elif self.settings.arch == "armv8":
            tools.download(
                f"https://developer.download.nvidia.com/compute/cuda/{self.version}/local_installers/cuda_{self.version}_{nv_version}_linux_sbsa.run",
                filename=f"cuda.run",
            )

        tmp_dir = os.path.join(self.build_folder, "tmp")
        os.mkdir(tmp_dir)
        self.run(
            f'sh cuda.run  --silent --override --override-driver-check --tmpdir={tmp_dir} --extract="{self.build_folder}"'
        )
        os.remove("cuda.run")
        self.run(f"sh NVIDIA-Linux-{nv_arch}-{nv_version}.run --extract-only")
        os.remove(f"NVIDIA-Linux-{nv_arch}-{nv_version}.run")
        self.run("rm -rf libcublas")
        self.run("rm -rf cuda-samples")

    def package(self):
        arch = arch_map[str(self.settings.arch)]
        self.copy("nvvm")
        self.copy("*", dst="bin", src="cuda_nvcc/bin")
        self.copy("*", dst="bin", src="cuda_nvcc/nvvm/bin")
        self.copy("*", dst="lib", src=f"cuda_cudart/targets/{arch}-linux/lib")
        self.copy("*.h*", dst="include", src=f"cuda_cudart/targets/{arch}-linux/include")
        self.copy("*.h*", dst="include", src=f"cuda_nvcc/targets/{arch}-linux/include")
        self.copy("*.h*", dst="include", src=f"libcurand/targets/{arch}-linux/include")
        self.copy("*.bc", src="cuda_nvcc")
        self.copy("*libcuda.so*", dst="lib", keep_path=False)
        self.copy("*libnvcuvid.so*", dst="lib", keep_path=False)
        with tools.chdir(os.path.join(self.package_folder, "lib")):
            os.symlink(f"libnvcuvid.so.{driver_map[self.version]}", "libnvcuvid.so.1")
            os.symlink("libnvcuvid.so.1", "libnvcuvid.so")
        self.copy(pattern="*.pc", dst="lib/pkgconfig")

    def package_info(self):
        self.env_info.CUDACXX = "clang++"
        # NVIDIA archs: https://arnon.dk/matching-sm-architectures-arch-and-gencode-for-various-nvidia-cards/
        gpu_arch = "sm_60" if self.settings.arch == "x86_64" else "sm_50"
        self.env_info.CUDAFLAGS = f" --cuda-gpu-arch={gpu_arch} -I{os.path.join(self.package_folder, 'include')} --cuda-path={self.package_folder}"

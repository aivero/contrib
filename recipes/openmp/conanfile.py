from build import *


class Openmp(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "LLVM OpenMP Runtime Library"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
        "libffi/[^3.3]",
        "elfutils/[>=0.179]",
        "perl/[^5.30.0]",
    )

    def source(self):
        self.get(
            f"https://github.com/llvm/llvm-project/releases/download/llvmorg-{self.version}/openmp-{self.version}.src.tar.xz"
        )

    def build(self):
        defs = {
            "LIBOMP_ENABLE_SHARED": True,
            "OPENMP_ENABLE_LIBOMPTARGET": False,
            "LIBOMP_OMPT_SUPPORT": False,
        }
        self.cmake(defs)

    def package_info(self):
        self.env_info.CPATH.append(os.path.join(self.package_folder, "include"))

from build import *


class Lld(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Linker from the LLVM project"
    license = "custom"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )

    def requirements(self):
        self.requires(f"llvm/[^{self.settings.compiler.version}]")

    def source(self):
        prefix = "https://github.com/llvm/llvm-project/releases/download/llvmorg-"
        self.get(f"{prefix}{self.version}/lld-{self.version}.src.tar.xz")
        self.get(f"{prefix}{self.version}/llvm-{self.version}.src.tar.xz", "llvm.src")
        self.get(f"{prefix}{self.version}/libunwind-{self.version}.src.tar.xz", "libunwind.src")

    def build(self):
        defs = {
            "LLVM_LINK_LLVM_DYLIB": True,
            "LLVM_MAIN_SRC_DIR": os.path.join(self.build_folder, "llvm.src"),
            "LLVM_EXTERNAL_LIBUNWIND_SOURCE_DIR": os.path.join(self.build_folder, "libunwind.src"),
        }
        self.cmake(defs)

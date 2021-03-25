from build import *


class Lldb(Recipe):
    description = "Next generation, high-performance debugger"
    license = "Apache"
    options = {}
    default_options = {}

    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )
    requires = (
        "llvm/[^11.0.1]",
        "libedit/20190324-3.1",
        "python/[^3.8.5]",
        "python-six/[^1.15.0]",
    )

    def source(self):
        self.get(f"https://github.com/llvm/llvm-project/releases/download/llvmorg-{self.version}/llvm-{self.version}.src.tar.xz")
        self.get(f"https://github.com/llvm/llvm-project/releases/download/llvmorg-{self.version}/clang-{self.version}.src.tar.xz", os.path.join(self.src, "projects", "clang"))
        self.get(f"https://github.com/llvm/llvm-project/releases/download/llvmorg-{self.version}/lldb-{self.version}.src.tar.xz", os.path.join(self.src, "projects", "lldb"))

    def build(self):
        targets = [
            "install-lldb",
            "install-lldb-server",
            "install-liblldb",
        ]
        self.cmake(
            targets=targets,
        )
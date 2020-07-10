from conans import *


class LlvmBootstrapConan(ConanFile):
    name = "llvm-bootstrap"
    description = "Collection of modular and reusable compiler and toolchain technologies"
    license = "Apache"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    build_requires = (
        "cmake-bootstrap/[^3.17.3]",
        "ninja-bootstrap/[^1.10.0]",
    )
    requires = (
        "llvm-libunwind-bootstrap/[^10.0.0]",
        "libffi-bootstrap/[^3.3]",
        "ncurses-bootstrap/[^6.2]",
    )

    def source(self):
        tools.get(f"https://github.com/llvm/llvm-project/releases/download/llvmorg-{self.version}/llvm-{self.version}.src.tar.xz")

    def build(self):
        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch_build)]
        cmake = CMake(self, generator="Ninja")
        cmake.definitions["LLVM_BUILD_LLVM_DYLIB"] = True
        # cmake.definitions["LLVM_LINK_LLVM_DYLIB"] = True
        cmake.definitions["BUILD_SHARED_LIBS"] = True
        cmake.definitions["LLVM_INSTALL_UTILS"] = True
        cmake.definitions["LLVM_ENABLE_FFI"] = True
        cmake.definitions["LLVM_ENABLE_RTTI"] = True
        cmake.definitions["LLVM_INSTALL_BINUTILS_SYMLINKS"] = True
        cmake.definitions["LLVM_DEFAULT_TARGET_TRIPLE"] = f"{arch}-linux-gnu"
        cmake.configure(source_folder=f"llvm-{self.version}.src")
        cmake.build()
        cmake.install()

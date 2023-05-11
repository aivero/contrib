from build import *
import multiprocessing


class LlvmLibs(CppRecipe):
    description = "Collection of modular and reusable compiler and toolchain technologies"
    license = "custom"
    exports = ("disable-system-libs.patch",)
    build_requires = (
        "cmake-bootstrap/[^3.18.4]",
        "ninja-bootstrap/[^1.10.0]",
    )
    requires = "file/[^5.39]"

    def source(self):
        prefix = "https://github.com/llvm/llvm-project/releases/download/llvmorg-"
        self.get(
            f"{prefix}{self.version}/llvm-{self.version}.src.tar.xz", os.path.join(self.src, "llvm")
        )
        self.get(
            f"{prefix}{self.version}/clang-{self.version}.src.tar.xz",
            os.path.join(self.src, "clang"),
        )
        self.get(
            f"{prefix}{self.version}/lld-{self.version}.src.tar.xz", os.path.join(self.src, "lld")
        )
        self.get(
            f"{prefix}{self.version}/compiler-rt-{self.version}.src.tar.xz",
            os.path.join(self.src, "compiler-rt"),
        )
        self.get(
            f"{prefix}{self.version}/libcxx-{self.version}.src.tar.xz",
            os.path.join(self.src, "libcxx"),
        )
        self.get(
            f"{prefix}{self.version}/libcxxabi-{self.version}.src.tar.xz",
            os.path.join(self.src, "libcxxabi"),
        )
        self.get(
            f"{prefix}{self.version}/libunwind-{self.version}.src.tar.xz",
            os.path.join(self.src, "libunwind"),
        )
        self.patch("disable-system-libs.patch")

    def build(self):
        source_folder = os.path.join(self.src, "llvm")
        defs = {}

        # Enable parallel linking
        defs["LLVM_PARALLEL_LINK_JOBS"] = multiprocessing.cpu_count()

        # LLVM build options
        if self.settings.arch == "x86_64":
            defs["LLVM_TARGETS_TO_BUILD"] = "X86;WebAssembly;AArch64;NVPTX;AMDGPU"
            arch = "x86_64"
        elif self.settings.arch == "armv8":
            defs["LLVM_TARGETS_TO_BUILD"] = "AArch64;NVPTX"
            arch = "aarch64"

        if self.settings.libc == "musl":
            abi = "musl"
        else:
            abi = "gnu"

        defs["LLVM_EXTERNAL_CLANG_SOURCE_DIR"] = os.path.join(self.build_folder, self.src, "clang")
        defs["LLVM_EXTERNAL_LLD_SOURCE_DIR"] = os.path.join(self.build_folder, self.src, "lld")
        defs["LLVM_EXTERNAL_COMPILER_RT_SOURCE_DIR"] = os.path.join(
            self.build_folder, self.src, "compiler-rt"
        )
        defs["LLVM_EXTERNAL_LIBCXX_SOURCE_DIR"] = os.path.join(
            self.build_folder, self.src, "libcxx"
        )
        defs["LLVM_EXTERNAL_LIBCXXABI_SOURCE_DIR"] = os.path.join(
            self.build_folder, self.src, "libcxxabi"
        )
        defs["LLVM_EXTERNAL_LIBUNWIND_SOURCE_DIR"] = os.path.join(
            self.build_folder, self.src, "libunwind"
        )

        defs["LLVM_HOST_TRIPLE"] = f"{arch}-unknown-linux-{abi}"

        defs["LLVM_ENABLE_PIC"] = True
        defs["LLVM_BUILD_RUNTIME"] = True
        defs["LLVM_BUILD_DOCS"] = False
        defs["LLVM_BUILD_EXAMPLES"] = False
        defs["LLVM_BUILD_TESTS"] = False

        # Build and link all libs as shared
        defs["LLVM_BUILD_LLVM_DYLIB"] = True
        defs["LLVM_LINK_LLVM_DYLIB"] = True
        defs["CLANG_LINK_CLANG_DYLIB"] = True
        defs["BUILD_SHARED_LIBS"] = False

        # LLVM enable options
        defs["LLVM_ENABLE_LIBCXX"] = True
        defs["LLVM_ENABLE_RTTI"] = True
        defs["LLVM_ENABLE_ZLIB"] = True
        defs["LLVM_ENABLE_Z3_SOLVER"] = False
        defs["LLVM_ENABLE_TERMINFO"] = False
        defs["LLVM_ENABLE_FFI"] = False
        defs["LLVM_ENABLE_LIBXML2"] = False
        defs["LLVM_ENABLE_SPHINX"] = False

        # LLVM other options
        defs["LLVM_INCLUDE_EXAMPLES"] = False
        defs["LLVM_INSTALL_BINUTILS_SYMLINKS"] = True
        defs["LLVM_INSTALL_UTILS"] = True

        # clang options
        defs["CLANG_VENDOR"] = "Aivero"
        defs["CLANG_DEFAULT_LINKER"] = "lld"
        defs["CLANG_DEFAULT_OBJCOPY"] = "llvm-objcopy"
        defs["CLANG_DEFAULT_UNWINDLIB"] = "libgcc"
        defs["CLANG_ENABLE_STATIC_ANALYZER"] = True

        # libcxx
        defs["CLANG_DEFAULT_CXX_STDLIB"] = "libc++"
        defs["CLANG_DEFAULT_RTLIB"] = "compiler-rt"

        # compiler-rt options
        defs["COMPILER_RT_BUILD_SANITIZERS"] = False
        defs["COMPILER_RT_BUILD_XRAY"] = False
        defs["COMPILER_RT_BUILD_LIBFUZZER"] = False

        # libcxx options
        defs["LIBCXX_ENABLE_SHARED"] = False
        defs["LIBCXX_ENABLE_STATIC_ABI_LIBRARY"] = True
        defs["LIBCXX_USE_COMPILER_RT"] = True
        if self.settings.libc == "musl":
            defs["LIBCXX_HAS_MUSL_LIBC"] = True

        # libcxxabi options
        defs["LIBCXXABI_ENABLE_SHARED"] = False
        defs["LIBCXXABI_USE_COMPILER_RT"] = True

        # libunwind
        defs["LIBUNWIND_ENABLE_SHARED"] = False

        # LTO
        defs["LLVM_ENABLE_LTO"] = "Thin"

        targets = [
            "install-libclang",
            "install-clang-cpp",
            "install-clang-resource-headers",
            "install-clang-headers",
            "install-cmake-exports",
            "install-strip",
            "install-strings",
            "install-readelf",
            "install-objcopy",
            "install-objdump",
            "install-nm",
            "install-llvm-as",
            "install-llvm-config",
            "install-llvm-tblgen",
            "install-llvm-profdata",
            "install-FileCheck",
            "install-llvm-libraries",
            "install-llvm-headers",
            "install-llvm-dwp",
        ]
        self.cmake(
            defs,
            targets=targets,
            source_folder=source_folder,
            build_folder=f"stage2-{self.version}",
        )

        # Create linker script that point libstdc++ to libc++
        with open(os.path.join(self.package_folder, "lib", "libstdc++.so"), "w") as lib:
            lib.write("GROUP ( -lc++ )")

    def package_info(self):
        self.env_info.CPP = os.path.join(self.package_folder, "bin", "clang-cpp")
        self.env_info.AS = os.path.join(self.package_folder, "bin", "llvm-as")
        self.env_info.LD = os.path.join(self.package_folder, "bin", "ld")
        self.env_info.STRIP = os.path.join(self.package_folder, "bin", "strip")
        self.env_info.OBJCOPY = os.path.join(self.package_folder, "bin", "objcopy")

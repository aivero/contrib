from build import *
import multiprocessing


class Llvm(Recipe):
    description = "Collection of modular and reusable compiler and toolchain technologies"
    license = "custom"
    exports = ("*-disable-system-libs.patch",)
    build_requires = (
        "cmake-bootstrap/[^3.18.4]",
        "ninja-bootstrap/[^1.10.0]",
        "libc/[^1.0.0]",
    )
    requires = "file/[^5.39]"

    def source(self):
        prefix = f"https://github.com/llvm/llvm-project/releases/download/llvmorg-{self.version}"
        self.get(f"{prefix}/llvm-project-{self.version}.src.tar.xz", self.src)
        self.patch(f"{self.version}-disable-system-libs.patch")

    def build(self):
        source_folder = os.path.join(self.src, "llvm")
        defs = {}

        os.environ["CC"] = "clang"
        os.environ["CXX"] = "clang++"

        # Enable parallel linking
        defs["LLVM_PARALLEL_LINK_JOBS"] = multiprocessing.cpu_count()

        # LLVM build options
        if self.settings.arch == "x86_64":
            defs["LLVM_TARGETS_TO_BUILD"] = "X86"
            arch = "x86_64"
        elif self.settings.arch == "armv8":
            defs["LLVM_TARGETS_TO_BUILD"] = "AArch64"
            arch = "aarch64"
        if self.settings.libc == "musl":
            abi = "musl"
        else:
            abi = "gnu"

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

        defs["LLVM_ENABLE_RUNTIMES"] = "libcxx;libcxxabi;libunwind"

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

        defs["LIBUNWIND_ENABLE_SHARED"] = False

        ###########
        # Stage 0 #
        ###########

        # Install stage 0 to build directory
        stage0_folder = os.path.join(self.build_folder, f"stage0-{self.version}-install")
        defs["CMAKE_INSTALL_PREFIX"] = stage0_folder

        # Reduce memory footprint of linking with gold linker
        defs["LLVM_USE_LINKER"] = "gold"

        # Stage 0 build (lld, clang, ar)
        targets = [
            "install-cxx",
            "install-compiler-rt",
            "install-clang",
            "install-clang-cpp",
            "install-clang-resource-headers",
            "install-llvm-libraries",
            "install-ar",
            "install-ranlib",
            "install-strip",
            "install-lld",
            "install-llvm-tblgen",
        ]
        self.cmake(
            defs,
            targets=targets,
            source_folder=source_folder,
            build_folder=f"stage0-{self.version}",
        )

        # Use stage 0 libs
        os.environ["LD_LIBRARY_PATH"] = os.path.join(stage0_folder, "lib")

        # Use stage 0 lld, clang, ar and ranlib
        defs["LLVM_USE_LINKER"] = os.path.join(stage0_folder, "bin", "ld.lld")
        defs["CMAKE_C_COMPILER"] = os.path.join(stage0_folder, "bin", "clang")
        defs["CMAKE_CXX_COMPILER"] = os.path.join(stage0_folder, "bin", "clang++")
        defs["CMAKE_AR"] = os.path.join(stage0_folder, "bin", "ar")
        defs["CMAKE_RANLIB"] = os.path.join(stage0_folder, "bin", "ranlib")
        defs["LLVM_TABLEGEN"] = os.path.join(stage0_folder, "bin", "llvm-tblgen")

        # Stage 0 clang can actually create useful LTO libraries
        defs["LLVM_ENABLE_LTO"] = "Thin"

        ###########
        # Stage 1 #
        ###########

        # Only needed for building libcxx
        cflags = ""
        # Install stage 1 to build directory
        stage1_folder = os.path.join(self.build_folder, f"stage1-{self.version}-install")
        defs["CMAKE_INSTALL_PREFIX"] = stage1_folder

        # Statically link everything with musl
        if self.settings.libc == "musl":
            cflags = "-static"

        libcxx_lib = os.path.join(stage0_folder, "lib")
        os.environ["LIBRARY_PATH"] = libcxx_lib
        os.environ["CFLAGS"] = cflags
        os.environ["CXXLAGS"] = cflags
        os.environ["LDFLAGS"] = cflags

        # Stage 1 build (cxx, cxxabi)
        self.cmake(
            defs,
            targets=[
                "install-cxx",
                "install-compiler-rt",
            ],
            source_folder=source_folder,
            build_folder=f"stage1-{self.version}",
        )

        ###########
        # Stage 2 #
        ###########

        # Install stage 2 to package directory
        defs["CMAKE_INSTALL_PREFIX"] = self.package_folder
        defs["CMAKE_SYSTEM_NAME"] = "Linux"

        # Use stage 1 libs and incs
        ldflags = ""
        # GVN causes segmentation fault during recursion higher than 290
        if self.settings.libc == "musl":
            ldflags = "-Wl,-mllvm,-gvn-max-recurse-depth=250"
        libcxx_inc = os.path.join(stage1_folder, "include", "c++", "v1")
        libcxx_lib = os.path.join(stage1_folder, "lib")
        clang_inc = os.path.join(stage1_folder, "lib", "clang", self.version, "include")
        clang_lib = os.path.join(stage1_folder, "lib", "clang", self.version, "lib", "linux")
        libc_inc = self.env["LIBC_INCLUDE_PATH"]
        os.environ[
            "CXXFLAGS"
        ] = f"{cflags} -idirafter {libcxx_inc} -idirafter {clang_inc} -idirafter {libc_inc}"
        os.environ["LDFLAGS"] = f"{cflags} {ldflags} -L{clang_lib} -L{libcxx_lib}"
        os.environ["LIBRARY_PATH"] = libcxx_lib

        # Enable additional targets
        if self.settings.arch == "x86_64":
            defs["LLVM_TARGETS_TO_BUILD"] = "X86;WebAssembly;AArch64;NVPTX;AMDGPU"
            arch = "x86_64"
        elif self.settings.arch == "armv8":
            defs["LLVM_TARGETS_TO_BUILD"] = "AArch64;NVPTX"
            arch = "aarch64"

        targets = [
            "install-cxx",
            "install-compiler-rt",
            "install-libclang",
            "install-clang",
            "install-clang-cpp",
            "install-clang-resource-headers",
            "install-clang-headers",
            "install-cmake-exports",
            "install-ar",
            "install-ranlib",
            "install-strip",
            "install-strings",
            "install-readelf",
            "install-objcopy",
            "install-objdump",
            "install-nm",
            "install-lld",
            "install-llvm-as",
            "install-llvm-config",
            "install-llvm-tblgen",
            "install-llvm-profdata",
            "install-FileCheck",
            "install-llvm-libraries",
            "install-llvm-headers",
            "install-llvm-dwp",
        ]
        # Stage 2 build (lld, clang, libcxx, libcxxabi)
        self.cmake(
            defs,
            targets=targets,
            source_folder=source_folder,
            build_folder=f"stage2-{self.version}",
        )

        # Create linker script that point libstdc++ to libc++
        with open(os.path.join(self.package_folder, "lib", "libstdc++.so"), "w") as lib:
            lib.write("GROUP ( -lc++ )")

        # Make lld, clang, clang++, clang-cpp default
        with tools.chdir(os.path.join(self.package_folder, "bin")):
            os.symlink("ld.lld", "ld")
            os.symlink("clang", "cc")
            os.symlink("clang++", "c++")
            os.symlink("clang-cpp", "cpp")

        # Delete component libs (They are part of the shared libs)
        if self.options.shared:
            for lib in os.listdir(os.path.join(self.package_folder, "lib")):
                if lib.endswith(".a") and lib != "libc++.a":
                    os.remove(os.path.join(self.package_folder, "lib", lib))

    def package_info(self):
        self.env_info.CC = os.path.join(self.package_folder, "bin", "clang")
        self.env_info.CXX = os.path.join(self.package_folder, "bin", "clang++")
        self.env_info.CPP = os.path.join(self.package_folder, "bin", "clang-cpp")
        self.env_info.AR = os.path.join(self.package_folder, "bin", "ar")
        self.env_info.AS = os.path.join(self.package_folder, "bin", "llvm-as")
        self.env_info.RANLIB = os.path.join(self.package_folder, "bin", "ranlib")
        self.env_info.LD = os.path.join(self.package_folder, "bin", "ld")
        self.env_info.STRIP = os.path.join(self.package_folder, "bin", "strip")
        self.env_info.OBJCOPY = os.path.join(self.package_folder, "bin", "objcopy")

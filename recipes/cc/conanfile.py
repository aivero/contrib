from build import *


class CC(CppRecipe):
    description = "Virtual c/c++ compiler package"
    license = "MIT"

    def requirements(self):
        self.requires("libc/[^1.0.0]")
        self.requires(f"llvm/[^{self.settings.compiler.version}]")

    def package_info(self):
        if self.settings.libc == "musl":
            static_flags = "-static"
        else:
            static_flags = ""
        if self.settings.arch == "x86_64":
            arch = "x86_64"
        elif self.settings.arch == "armv8":
            arch = "aarch64"
        if self.settings.libc == "musl":
            abi = "musl"
        else:
            abi = "gnu"

        triple = f"{arch}-unknown-linux-{abi}"

        llvm_deps_cpp_info = self.deps_cpp_info["llvm"]
        llvm_rootpath = llvm_deps_cpp_info.rootpath
        libc_inc = self.env["LIBC_INCLUDE_PATH"]
        libclang_inc = os.path.join(
            llvm_rootpath,
            "lib",
            "clang",
            llvm_deps_cpp_info.version,
            "include",
        )
        llvm_inc = os.path.join(llvm_rootpath, "include")
        libcxx_inc = os.path.join(llvm_rootpath, "include", "c++", "v1")
        libcxx_target_inc = os.path.join(llvm_rootpath, "include", triple, "c++", "v1")

        # -Wno-unused-command-line-argument is needed for some sanity tests in cmake
        cflags = f" -nostdinc -idirafter {libclang_inc} -idirafter {libc_inc} -idirafter {llvm_inc} {static_flags} -fPIC -Wno-unused-command-line-argument "
        cxxflags = f" -nostdinc++ -idirafter {libcxx_inc} -idirafter {libcxx_target_inc} {cflags} "

        self.env_info.CFLAGS = cflags
        self.env_info.CXXFLAGS = cxxflags

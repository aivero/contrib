import os

from conans import *

musl_clang = """#!/usr/bin/env sh
# prevent clang from running the linker (and erroring) on no input.
sflags=
eflags=
for x ; do
    case "$x" in
        -l*) input=1 ;;
        *) input= ;;
    esac
    if test "$input" ; then
        sflags="-l-user-start"
        eflags="-l-user-end"
        break
    fi
done

exec clang \
    -fuse-ld=musl-clang \
    -static-libgcc \
    -nostdinc \
    --sysroot "$MUSL_SYSROOT" \
    -isystem "$MUSL_INCLUDE_PATH" \
    -L-user-start \
    $sflags \
    "$@" \
    $eflags \
    -L"$MUSL_LIBRARY_PATH" \
    -L-user-end
"""

musl_clangpp = """#!/usr/bin/env sh
# prevent clang from running the linker (and erroring) on no input.
sflags=
eflags=
for x ; do
    case "$x" in
        -l*) input=1 ;;
        *) input= ;;
    esac
    if test "$input" ; then
        sflags="-l-user-start"
        eflags="-l-user-end"
        break
    fi
done

exec clang++ \
    -fuse-ld=musl-clang \
    -nostdinc \
    -nostdlib++ \
    --sysroot "$MUSL_SYSROOT" \
    -isystem "$MUSL_INCLUDE_PATH" \
    -L-user-start \
    $sflags \
    "$@" \
    $eflags \
    -L"$MUSL_LIBRARY_PATH" \
    -L-user-end
"""


class MuslConan(ConanFile):
    name = "musl"
    description = "Lightweight implementation of C standard library"
    license = "MIT"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    build_requires = ("clang-bootstrap/[^10.0.0]",)

    def source(self):
        tools.get(f"https://www.musl-libc.org/releases/musl-{self.version}.tar.gz")

    def build(self):
        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch_build)]
        vars = {"CC": "clang", "TARGET": f"{arch}-linux-musl"}
        autotools = AutoToolsBuildEnvironment(self)
        autotools.configure(vars=vars, configure_dir=f"{self.name}-{self.version}")
        autotools.make()
        autotools.install()
        with tools.chdir(os.path.join(self.package_folder, "bin")):
            os.symlink(os.path.join("..", "lib", "libc.so"), "ldd")
        with tools.chdir(os.path.join(self.package_folder, "lib")):
            os.symlink(os.path.join("..", "lib", "libc.so"), f"ld-musl-{arch}.so.1")

        with open("musl-clang", "w") as pc:
            pc.write(musl_clang)
        os.chmod("musl-clang", 0o775)
        with open("musl-clang++", "w") as pc:
            pc.write(musl_clangpp)
        os.chmod("musl-clang++", 0o775)

    def package(self):
        self.copy("musl-clang*", dst="bin", keep_path=False)

    def package_info(self):
        self.env_info.MUSL_SYSROOT = self.package_folder
        self.env_info.MUSL_LIBRARY_PATH = os.path.join(self.package_folder, "lib")
        self.env_info.MUSL_INCLUDE_PATH = os.path.join(self.package_folder, "include")

from build import *


class Glibc(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "GNU C Library"
    license = "GPL"
    options = {}
    default_options = {}
    requires = "linux-headers/[^5.4.90]"

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/glibc/glibc-{self.version}.tar.xz")

    def build(self):
        self.autotools(target="install-headers")

    def package(self):
        # install-headers does not create include/gnu/stubs.h
        pathlib.Path(os.path.join(self.package_folder, "include", "gnu", "stubs.h")).touch()

        # Copy glibc and gcc libs
        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch)]
        lib_dir = os.path.join(self.package_folder, "lib-dev")
        os.makedirs(lib_dir)
        with tools.chdir(lib_dir):
            # Copy from glibc-dev
            libs = [
                "libdl.so",
                "librt.so",
                "libresolv.so",
                "libutil.so",
                "libatomic.so.1",
                "libpthread.so",
                "crt1.o",
                "crti.o",
                "crtn.o",
                "Scrt1.o",
                "libc_nonshared.a",
            ]
            for lib in libs:
                shutil.copy2(f"/usr/lib/{arch}-linux-gnu/{lib}", lib)
            # Copy and fix linker scripts from glibc-dev
            ld_scripts = [
                "libc.so",
            ]
            # libm.so is not a linker script on aarch64
            if arch == "x86_64":
                ld_scripts.append("libm.so")
            else:
                os.symlink("libm.so.6", "libm.so")
            for ld_script in ld_scripts:
                shutil.copy2(f"/usr/lib/{arch}-linux-gnu/{ld_script}", ld_script)
                tools.replace_path_in_file(
                    ld_script, f"/usr/lib/{arch}-linux-gnu/", "", strict=False
                )
                tools.replace_path_in_file(ld_script, f"/lib/{arch}-linux-gnu/", "")
            # Copy base glibc and fix linker scripts
            libs = [
                "libc.so.6",
                "libm.so.6",
                "libpthread.so.0",
                "libdl.so.2",
                "librt.so.1",
                "libresolv.so.2",
                "libutil.so.1",
                "libgcc_s.so.1",
                "libselinux.so.1",
                "libpcre.so.3",
            ]
            if arch == "x86_64":
                libs.append("libmvec.so.1")
            for lib in libs:
                shutil.copy2(f"/lib/{arch}-linux-gnu/{lib}", lib)
                for ld_script in ld_scripts:
                    tools.replace_path_in_file(
                        ld_script, f"/lib/{arch}-linux-gnu/{lib}", lib, strict=False
                    )
            # Copy files from libgcc-9-dev
            libs = [
                "libgcc_s.so",
                "libgcc.a",
                "libatomic.so",
                "crtbegin.o",
                "crtend.o",
            ]
            for lib in libs:
                shutil.copy2(f"/usr/lib/gcc/{arch}-linux-gnu/9/{lib}", lib)
            # Copy linker lib
            if arch == "x86_64":
                lib = "ld-linux-x86-64.so.2"
                shutil.copy2(f"/lib64/{lib}", lib)
            elif arch == "aarch64":
                lib = "ld-linux-aarch64.so.1"
                shutil.copy2(f"/lib/{lib}", lib)

    def package_info(self):
        self.env_info.LIBC_INCLUDE_PATH = os.path.join(self.package_folder, "include")
        self.env_info.LIBC_LIBRARY_PATH = os.path.join(self.package_folder, "lib-dev")
        self.env_info.CMAKE_LIBRARY_PATH = os.path.join(self.package_folder, "lib-dev")

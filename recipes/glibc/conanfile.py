from build import *


class GlibcRecipe(Recipe):
    description = "GNU C Library"
    license = "GPL"
    options = {}
    default_options = {}
    requires = "linux-headers/[^5.4.50]"

    def source(self):
        self.get(f"https://ftp.gnu.org/gnu/glibc/glibc-{self.version}.tar.xz")

    def build(self):
        self.autotools(target="install-headers")

    def package(self):
        # install-headers does not create include/gnu/stubs.h
        pathlib.Path(os.path.join(self.package_folder, "include", "gnu", "stubs.h")).touch()

        # Use system libgcc_s
        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch)]
        os.makedirs(os.path.join(self.package_folder, "lib"))
        with tools.chdir(os.path.join(self.package_folder, "lib")):
            # Copy from glibc-dev
            libs = [
                "libdl.so",
                "librt.so",
                "libresolv.so",
                "libutil.so",
                "crt1.o",
                "crti.o",
                "crtn.o",
                "Scrt1.o",
                "libc_nonshared.a",
                "libpthread_nonshared.a",
            ]
            if arch == "x86_64":
                libs.append("libmvec_nonshared.a")
            for lib in libs:
                shutil.copy2(f"/usr/lib/{arch}-linux-gnu/{lib}", lib)
            # Copy and fix linker scripts from glibc-dev
            ld_scripts = [
                "libc.so",
                "libm.so",
                "libpthread.so",
            ]
            for ld_script in ld_scripts:
                # libm.so is no a linker script on aarch64
                if arch == "aarch64" and ld_script == "libm.so":
                    os.symlink("libm.so.6", "libm.so")
                else:
                    shutil.copy2(f"/usr/lib/{arch}-linux-gnu/{ld_script}", ld_script)
                    tools.replace_path_in_file(ld_script, f"/usr/lib/{arch}-linux-gnu/", "")
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
            for lib in libs:
                shutil.copy2(f"/lib/{arch}-linux-gnu/{lib}", lib)
                for ld_script in ld_scripts:
                    tools.replace_path_in_file(ld_script, f"/lib/{arch}-linux-gnu/{lib}", lib, strict=False)
            # Symlink files from libatomic1
            libs = [
                "libatomic.so.1",
            ]
            for lib in libs:
                os.symlink(f"/usr/lib/{arch}-linux-gnu/{lib}", lib)
            # Copy files from libgcc-7-dev
            libs = [
                "libgcc_s.so",
                "libgcc.a",
                "libatomic.so",
            ]
            for lib in libs:
                shutil.copy2(f"/usr/lib/gcc/{arch}-linux-gnu/7/{lib}", lib)
            # Copy linker lib
            if arch == "x86_64":
                lib = "ld-linux-x86-64.so.2"
                shutil.copy2(f"/lib64/{lib}", lib)
            elif arch == "aarch64":
                lib = "ld-linux-aarch64.so.1"
                shutil.copy2(f"/lib/{lib}", lib)

    def package_info(self):
        self.env_info.LIBC_LIBRARY_PATH = os.path.join(self.package_folder, "lib")
        self.env_info.LIBC_INCLUDE_PATH = os.path.join(self.package_folder, "include")

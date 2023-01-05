from build import *


class Libatomic(Recipe):
    description = "GNU libatomic"
    license = "GPL"
    options = {}
    default_options = {}

    def package(self):
        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch)]
        lib_dir = os.path.join(self.package_folder, "lib")
        os.makedirs(lib_dir)
        with tools.chdir(lib_dir):
            # Copy files from libgcc-7-dev
            libs = [
                "libatomic.so.1",
            ]
            for lib in libs:
                shutil.copy2(f"/usr/lib/{arch}-linux-gnu/{lib}", lib)

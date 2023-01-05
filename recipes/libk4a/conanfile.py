from build import *


class Libk4a(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "Azure Kinect SDK"
    license = "MIT"
    exports = "k4a.pc"

    def source(self):
        version_short = self.version[:3]
        arch = self.settings.arch
        debian_repo_url = ""
        if arch == "x86_64":
            arch = "amd64"
            debian_repo_url = "https://packages.microsoft.com/ubuntu/18.04/prod/pool/main/libk"
        if arch == "armv8":
            arch = "arm64"
            debian_repo_url = (
                "https://packages.microsoft.com/ubuntu/18.04/multiarch/prod/pool/main/libk/"
            )

        libk4a = f"libk4a{version_short}_{self.version}_{arch}.deb"
        libk4a_dev = f"libk4a{version_short}-dev_{self.version}_{arch}.deb"

        # Download `libk4a` and `libk4a-dev` for headers and shared objects
        tools.download(f"{debian_repo_url}/libk4a{version_short}/{libk4a}", filename=libk4a)
        tools.download(
            f"{debian_repo_url}/libk4a{version_short}-dev/{libk4a_dev}", filename=libk4a_dev
        )

        # Extract shared objects, including the closed-source `libdepthengine.so*`
        self.run(f"dpkg -x {libk4a} libk4a")
        # Extract headers
        self.run(f"dpkg -x {libk4a_dev} libk4a")

    def package(self):
        # Architecture dependent lib dir
        lib_dir_arch = os.listdir("libk4a/usr/lib")
        tools.replace_prefix_in_pc_file("k4a.pc", self.package_folder)
        self.copy("*", src="libk4a/usr/include", dst="include")
        self.copy("*", src="libk4a/usr/lib/" + lib_dir_arch[0], dst="lib", symlinks=True)
        self.copy("k4a.pc", dst="lib/pkgconfig")

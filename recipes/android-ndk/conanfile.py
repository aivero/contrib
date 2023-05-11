from build import *


class AndroidNdk(CppRecipe):
    description = "Android NDK"
    license = "Apache"
    options = {}
    default_options = {}

    def source(self):
        major, minor = self.version.split(".")

        minor = chr(ord("a") + int(minor))
        tools.get(
            f"https://dl.google.com/android/repository/android-ndk-r{major}{minor}-linux-x86_64.zip"
        )

    def build(self):
        pass

    def package(self):
        arch = {"x86_64": "x86_64", "armv8": "arm64"}[str(self.settings.arch)]
        self.copy(pattern="*", src=f"android-ndk-r{self.ndk_version}", symlinks=True)

        # Fix permissions
        exe_dir = os.path.join(
            self.package_folder, "toolchains", "llvm", "prebuilt", "linux-x86_64", "bin"
        )
        for exe in os.listdir(exe_dir):
            exe_path = os.path.join(exe_dir, exe)
            os.chmod(exe_path, 0o775)

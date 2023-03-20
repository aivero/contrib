from build import *


class IosWebkitDebugProxy(Recipe):
    description = "A DevTools proxy (Chrome Remote Debugging Protocol) for iOS devices"
    license = "LGPL"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = ("libimobiledevice/[^1.3.0]",)

    def source(self):
        self.get(
            f"https://github.com/google/ios-webkit-debug-proxy/archive/refs/tags/v{self.version}.tar.gz"
        )

    def build(self):
        os.environ["CFLAGS"] += f"-Wno-deprecated-declarations -Wno-deprecated-non-prototype -I{os.path.join(self.source_folder, self.src, 'include', 'ios-webkit-debug-proxy')}"
        args = ['--disable-dependency-tracking']
        self.autotools(args)

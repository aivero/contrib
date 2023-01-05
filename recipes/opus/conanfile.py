from build import *


class Opus(Recipe):
    description = "Modern audio compression for the internet"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )

    def source(self):
        # CMake broken until next release (https://github.com/xiph/opus/issues/129)
        self.get(
            "https://github.com/xiph/opus/archive/034c1b61a250457649d788bbf983b3f0fb63f02e.tar.gz"
        )
        with open(os.path.join(self.src, "package_version"), "w") as file:
            file.write(f"PACKAGE_VERSION={self.version}")

from build import *


class LibcameraApps(Recipe):
    description = "small suite of libcamera based apps"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )
    requires = (
        "gst-libcamera/[>=0.0.4]",
        "libboost/[>=1.81.0]",
        "libexif/[^0.6.24]",
    )

    def source(self):
        self.get(f"https://github.com/raspberrypi/libcamera-apps/archive/refs/tags/v{self.version}.tar.gz")

    def build(self):
        for req in ["libdrm", "libboost", "libpng", "libjpeg-turbo", "libtiff", "libexif"]:
            os.environ[
                "CXXFLAGS"
            ] += f" -I{os.path.join(self.deps_cpp_info[req].rootpath, 'include')}"

        os.environ["CXXFLAGS"] += " -Wno-error  -Wno-nested-anon-types -Wno-c99-extensions"
        defs = {
            "ENABLE_DRM": True,
            "ENABLE_X11": True,
            "ENABLE_QT": False,
            "ENABLE_OPENCV": False,
            "ENABLE_TFLITE": False,
        }
        self.cmake(defs)

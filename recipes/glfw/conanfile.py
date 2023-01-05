from build import *


class Glfwc(Recipe):
    description = "GLFW is an Open Source, multi-platform library for OpenGL, OpenGL ES and Vulkan development on the desktop."
    license = "ZLIB"
    options = {"x11": [True, False]}
    default_options = ("x11=True",)
    build_requires = ("cc/[^1.0.0]", "cmake/[^3.15.3]")

    def requirements(self):
        if self.options.x11:
            self.requires("libxrandr/[^1.5.2]")
            self.requires("libxinerama/[^1.1.4]")
            self.requires("libxcursor/[^1.2.0]")
            self.requires("libxi/[^1.7.1]")

    def source(self):
        self.get(f"https://github.com/glfw/glfw/archive/{self.version}.tar.gz")

    def build(self):
        for req in ["libxext", "libxcb", "xorgproto", "libx11", "libxrender", "libxi", "libxfixes"]:
            os.environ[
                "CFLAGS"
            ] += f" -I{os.path.join(self.deps_cpp_info[req].rootpath, 'include')}"
        defs = {
            "BUILD_SHARED_LIBS": True,
            "GLFW_BUILD_EXAMPLES": False,
            "GLFW_BUILD_TESTS": False,
            "GLFW_BUILD_DOCS": False,
        }
        self.cmake(defs)

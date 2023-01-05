from build import *


class Sdl2(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "A library for portable low-level access to a video framebuffer, audio output, mouse, and keyboard"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
        "libxcb/[^1.13.1]",
        "libxext/[^1.3.4]",
        "xorgproto/[>=2020.1]",
    )

    def source(self):
        self.get(f"https://www.libsdl.org/release/SDL2-{self.version}.tar.gz")

    def build(self):
        for req in ["libxext", "libxcb", "xorgproto", "libx11"]:
            os.environ[
                "CFLAGS"
            ] += f" -I{os.path.join(self.deps_cpp_info[req].rootpath, 'include')}"
        self.cmake()

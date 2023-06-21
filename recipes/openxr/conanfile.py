from build import *


class Openxr(PythonRecipe):
    description = "An open standard for virtual reality and augmented reality platforms and devices"
    license = "Apache"
    build_requires = (
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
        "vulkan-headers/[^1.3.245]",
        "wayland/[^1.20.0]",
        "mesa/[^22.3.2]",
    )
    requires = (
        "jsoncpp/[^1.9.5]",
        "vulkan-icd-loader/[^1.3.245]",
        "libglvnd/[^1.3.2]",
        "libxrandr/[^1.5.2]",
        "libxxf86vm/[^1.1.4]",
    )

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/KhronosGroup/OpenXR-SDK-Source/releases/download/release-{self.version}/OpenXR-SDK-Source-release-{self.version}.tar.gz")

    def build(self):
        os.environ["CPATH"] += ":" + ":".join(
            self.deps_cpp_info["libx11"].include_paths
            + self.deps_cpp_info["xorgproto"].include_paths
            + self.deps_cpp_info["libxxf86vm"].include_paths
            + self.deps_cpp_info["libxrandr"].include_paths
            + self.deps_cpp_info["libxrender"].include_paths
            + self.deps_cpp_info["libxcb"].include_paths
            + self.deps_cpp_info["libglvnd"].include_paths
        )
        defs = {}
        self.cmake(defs)

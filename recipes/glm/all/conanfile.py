from conans import *
import os


class GlmConan(ConanFile):
    name = "glm"
    description = "OpenGL Mathematics (GLM) is a header only C++ mathematics library for graphics software based on the OpenGL Shading Language (GLSL) specifications."
    license = "MIT License"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    exports_sources = ["glm.pc"]

    def source(self):
        tools.get(f"https://github.com/g-truc/glm/releases/download/{self.version}/glm-{self.version}.zip")

    def package(self):
        self.copy(pattern="*", src="glm/glm", dst=os.path.join(self.package_folder, "include", "glm"), keep_path=True)
        self.copy(pattern="glm.pc", dst=os.path.join(self.package_folder, "lib", "pkgconfig"), keep_path=False)

from os.path import join
from build import *


class Tensorflow2Lite(CppRecipe):
    name = "tensorflow2-lite"
    description = "https://www.tensorflow.org"
    license = "LGPL"
    exports = ("tensorflow2-lite.pc", "0001-Force-libtensorflow-lite-to-be-shared.patch")
    build_requires = (
        "autotools/[^1.0.0]",
        "cc/[^1.0.0]",
        "cmake/[^3.18.4]",
    )

    def requirements(self):
        self.requires(f"flatbuffers/[^2.0.0]")

    def source(self):
        git = tools.Git(folder=f"{self.name}-{self.version}.src")
        git.clone("https://github.com/tensorflow/tensorflow.git", f"v{self.version}")

        # We apply this patch to force tensorflow-lite to create a shared library, even if
        # BUILD_SHARED_LIBS = Off.
        git.run(
            '-c user.email="cicd@civero.com" -c user.name="Chlorine Cadmium" am -3 '
            + "../0001-Force-libtensorflow-lite-to-be-shared.patch"
        )

    def build(self):
        defs = {
            # Set cmake to build static libraries so that tensorflow-lite is statically linked
            # with all the dependecies downloaded by cmake.
            "BUILD_SHARED_LIBS": False,
            "TFLITE_ENABLE_RUY": False,
            "TFLITE_ENABLE_RESOURCE": True,
            "TFLITE_ENABLE_NNAPI": False,
            "TFLITE_ENABLE_MMAP": False,
            "TFLITE_ENABLE_GPU": False,
            "TFLITE_ENABLE_FLEX": False,
            "TFLITE_ENABLE_XNNPACK": True,
        }
        self.cmake(defs, source_folder=os.path.join(self.src, "tensorflow/lite"))

    def package(self):
        tools.replace_prefix_in_pc_file("tensorflow2-lite.pc", self.package_folder)
        self.copy(pattern="*.pc", dst="lib/pkgconfig")
        self.copy(pattern="*.so", dst="lib", src=f".", keep_path=False)
        self.copy(
            pattern="*.h",
            dst="include/tensorflow/lite",
            src=join(self.src, "tensorflow/lite"),
            keep_path=True,
        )

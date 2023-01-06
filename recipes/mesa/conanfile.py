from build import *


class Mesa(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "An open-source implementation of the OpenGL specification"
    license = "custom"
    options = {
        "shared": [True, False],
        "x11": [True, False],
    }
    default_options = (
        "shared=True",
        "x11=True",
    )
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.55.3]",
        "gettext/[^0.21]",
        "bison/[^3.3]",
        "flex/[^2.6.4]",
        "python-mako/[^1.1.0]",
        "glslang/[^11.2.0]",
    )
    requires = (
        "libglvnd/[^1.3.2]",
        "zlib/[^1.2.11]",
        "expat/[^2.2.7]",
        "libdrm/[^2.4.114]",
    )

    def requirements(self):
        self.requires(f"llvm/[^{self.settings.compiler.version}]", "private")
        if self.options.x11:
            self.requires("libxrandr/[^1.5.2]")
            self.requires("libxdamage/[^1.1.5]")
            self.requires("libxshmfence/[^1.3]")
            self.requires("libxxf86vm/[^1.1.4]")

    def source(self):
        self.get(f"https://mesa.freedesktop.org/archive/mesa-{self.version}.tar.xz")

    def build(self):
        opts = {
            "glx": "dri",
            "glvnd": True,
            "egl": True,
            "gles2": True,
            "opengl": True,
        }
        if self.options.x11:
            opts["platforms"] = "x11"
        if self.settings.arch == "x86_64":
            opts["gallium-drivers"] = "iris,i915"
            opts["vulkan-drivers"] = "intel"
        if self.settings.arch == "armv8":
            opts["gallium-drivers"] = "nouveau,tegra,swrast"

        # For some reason, cicd fails when calling meson introspect on the mesa
        # package. Workaround here is to no do the opt_check
        self.meson(opts, opt_check=False)

    def package_info(self):
        self.env_info.LIBGL_DRIVERS_PATH.append(os.path.join(self.package_folder, "lib", "dri"))

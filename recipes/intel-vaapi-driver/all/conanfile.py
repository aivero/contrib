import os

from conans import ConanFile, Meson, tools


class IntelVaapiDriverConan(ConanFile):
    description = "VA-API user mode driver for Intel GEN Graphics family"
    license = "MIT"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    options = {"x11": [True, False], "wayland": [True, False]}
    default_options = ("x11=True", "wayland=False")

    def build_requirements(self):
        self.build_requires("generators/1.0.0")
        self.build_requires("meson/[^0.51.2]")

    def requirements(self):
        self.requires("libdrm/[^2.4.96]")
        self.requires("libva/[^2.3.0]")

    def source(self):
        tools.get("https://github.com/intel/intel-vaapi-driver/archive/%s.tar.gz" % self.version)

    def build(self):
        args = ["--auto-features=disabled"]
        args.append("-Dwith_x11=" + ("yes" if self.options.x11 else "no"))
        args.append("-Dwith_wayland=" + ("yes" if self.options.wayland else "no"))
        args.append("-Ddriverdir=" + os.path.join(self.package_folder, "lib", "dri"))
        meson = Meson(self)
        meson.configure(source_folder="%s-%s" % (self.name, self.version), args=args, pkg_config_paths=os.environ["PKG_CONFIG_PATH"].split(":"))
        meson.install()

    def package_info(self):
        self.env_info.LIBVA_DRIVERS_PATH.append(os.path.join(self.package_folder, "lib", "dri"))

from build import *


class Fontconfig(Recipe):
    description = "A library for configuring and customizing font access"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "meson/[>=0.61.1]",
        "gperf/[^3.1]",
    )
    requires = (
        "libuuid/[^1.0.3]",
        "freetype/[^2.10.3]",
        "expat/[^2.2.7]",
    )

    def source(self):
        self.get(
            f"https://www.freedesktop.org/software/fontconfig/release/fontconfig-{self.version}.tar.xz"
        )

    def package(self):
        with tools.chdir(f"{self.package_folder}/etc/fonts/conf.d"):
            for _, _, files in os.walk("."):
                for filename in files:
                    os.remove(filename)

        with tools.chdir(f"{self.package_folder}/etc/fonts/conf.d"):
            for _, _, files in os.walk(f"{self.package_folder}/share/fontconfig/conf.avail"):
                for filename in files:
                    os.symlink(f"../../../share/fontconfig/conf.avail/{filename}", filename)

    def package_info(self):
        self.env_info.FONTCONFIG_PATH.append(os.path.join(self.package_folder, "etc", "fonts"))

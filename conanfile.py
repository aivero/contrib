from conans import ConanFile, Meson, tools

class OrcConan(ConanFile):
    name = "orc"
    version = tools.get_env("GIT_TAG", "0.4.31")
    settings = "os", "compiler", "build_type", "arch"
    url = "https://gitlab.com/aivero/public/conan/conan-" + name
    license = "LGPL-2.1"
    description = "Optimized Inner Loop Runtime Compiler"
    generators = "env"

    def build_requirements(self):
        self.build_requires("env-generator/[>=1.0.0]@%s/stable" % self.user)
        self.build_requires("meson/[>=0.51.2]@%s/stable" % self.user)

    def source(self):
        tools.get("https://github.com/GStreamer/orc/archive/%s.tar.gz" % self.version)

    def build(self):
        args = ["-Dgtk_doc=disabled"]
        args.append("-Dbenchmarks=disabled")
        args.append("-Dexamples=disabled")
        meson = Meson(self)
        meson.configure(source_folder="orc-" + self.version, args=args)
        meson.install()

    def package(self):
        if self.settings.build_type == "Debug":
            self.copy("*.c", "src")
            self.copy("*.h", "src")

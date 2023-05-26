from build import *


class Nodejs(PythonRecipe):
    description = "Evented I/O for V8 javascript"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "autotools/[^1.0.0]",
    )
    requires = ("openssl1/[>=1.1.1h]", "zlib/[^1.2.11]", "libatomic/[^8.4.0]")

    def build_requirements(self):
        self.build_requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/nodejs/node/archive/v{self.version}.tar.gz")

    def build(self):
        args = [
            "--without-npm",
            "--shared-openssl",
            "--shared-zlib",
        ]
        self.autotools(args)

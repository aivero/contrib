from conans.errors import ConanInvalidConfiguration
from build import *


class PythonRecipe(PythonRecipe):
    description = "Next generation of the python high-level scripting language"
    license = "MIT"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
    )
    requires = (
        "libssl/[^1.0.0]",
        "expat/[^2.4.1]",
        "libffi/[^3.3]",
        "bzip2/[^1.0.8]",
        "sqlite/[^3.30.1]",
    )

    def validate(self):
        if str(self.settings.python) not in str(self.version):
            raise ConanInvalidConfiguration(
                f"Python version specified in devops.yml ({self.version}) is not compatible with version specified in profile: {self.settings.python}"
            )

    def source(self):
        self.get(f"https://www.python.org/ftp/python/{self.version}/Python-{self.version}.tar.xz")

    def build(self):
        # Python build scripts handles LTO
        env_replace("CFLAGS", "-flto=thin")

        args = [
            f"--with-openssl={self.deps_cpp_info['openssl'].rootpath}",
            "--with-computed-gotos",
            "--enable-optimizations",
            "--enable-ipv6",
            "--with-system-expat",
            "--with-system-ffi",
            "--enable-loadable-sqlite-extensions",
            "--without-ensurepip",
        ]
        if self.options.shared:
            args.append("--enable-shared")
        else:
            args.append("--with-lto")

        self.autotools(args)

        version = ".".join(self.version.split(".")[:2])
        os.symlink(f"python{version}", os.path.join(self.package_folder, "bin", "python"))

        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch)]
        with open(
            os.path.join(
                self.package_folder,
                "lib",
                f"python{version}",
                f"_sysconfigdata__linux_{arch}-linux-gnu.py",
            ),
            "w",
        ) as py:
            py.write(
                """import os
build_time_vars = {{
  "AR": os.environ.get("AR", ""),
  "ARFLAGS": "",
  "CC": os.environ.get("CC", ""),
  "CXX": os.environ.get("CXX", ""),
  "CPP": os.environ.get("CPP", ""),
  "AS": os.environ.get("AS", ""),
  "RANLIB": os.environ.get("RANLIB", ""),
  "LD": os.environ.get("LD", ""),
  "STRIP": os.environ.get("STRIP", ""),
  "OBJCOPY": os.environ.get("OBJCOPY", ""),
  "srcdir": "python.{0}.src",
  "EXT_SUFFIX": ".cpython-{1}{2}-{3}-linux-gnu.so",
  "CFLAGS": "",
  "CCSHARED": "",
  "LDSHARED": "",
  "LDFLAGS": "",
  "LDLIBRARY": "libpython{1}.{2}.so",
  "LIBPL": os.path.join(os.environ.get("PYTHONHOME", ""), "lib", "python{1}-{2}", "config-{1}-{2}-{3}-linux-gnu"),
  "LIBDIR": os.path.join(os.environ.get("PYTHONHOME", ""), "lib"),
  "SHLIB_SUFFIX": ".so",
  "LDCXXSHARED": os.environ.get("CXX", "") + " -shared -Wl,-O1 -Wl,-Bsymbolic-functions ",
  "LDSHARED": os.environ.get("CC", "") + " -shared -Wl,-O1 -Wl,-Bsymbolic-functions -Wl,-Bsymbolic-functions -Wl,-z,relro -g -fwrapv -O2 ", 
  "VERSION": "{1}.{2}",
  "LINKFORSHARED": "-Xlinker -export-dynamic",
}}
""".format(
                    self.version, self.version[0], self.version.split(".")[1], arch
                )
            )

    def package_info(self):
        self.env_info.PYTHON = os.path.join(self.package_folder, "bin", "python")
        self.env_info.VIRTUALENV_PYTHON = os.path.join(self.package_folder, "bin", "python")
        self.env_info.PYTHONHOME = self.package_folder
        if "CC" in os.environ:
            ldshared = os.environ["CC"] + " -pthread -shared "
            if self.settings.arch == "x86_64":
                ldshared += "-m64 "
            self.env_info.LDSHARED = ldshared

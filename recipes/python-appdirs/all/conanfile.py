import os

from conans import *


class PythonAppdirsConan(ConanFile):
    name = "python-appdirs"
    description = 'A small Python module for determining appropriate platform-specific dirs, e.g. a "user data dir".'
    license = "MIT"
    settings = {"os_build": ["Linux"], "arch_build": ["x86_64", "armv8"]}
    build_requires = ("python-setuptools/[^41.2.0]",)
    requires = (
        "base/[^1.0.0]",
        "python/[^3.7.4]",
    )

    def source(self):
        tools.get(f"https://pypi.io/packages/source/a/appdirs/appdirs-{self.version}.tar.gz")

    def build(self):
        with tools.chdir(f"appdirs-{self.version}"):
            self.run('python setup.py install --optimize=1 --prefix= --root="%s"' % self.package_folder)

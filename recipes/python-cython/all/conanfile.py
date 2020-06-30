import os

from conans import *


class PythonCythonConan(ConanFile):
    description = "Python to C compiler"
    license = "Apache"
    settings = {"os": ["Linux"], "arch": ["x86_64", "armv8"]}
    build_requires = (
        "generators/1.0.0",
        "gcc/[^7.4.0]",
        "pkgconf/[^1.6.3]",
        "python-setuptools/[^41.2.0]",
    )
    requires = ("python/[^3.7.4]",)

    def source(self):
        tools.get("https://github.com/cython/cython/archive/{0}.tar.gz".format(self.version))

    def build(self):
        with tools.chdir("cython-{0}".format(self.version)):
            self.run('python setup.py install --optimize=1 --prefix= --root="%s"' % self.package_folder)


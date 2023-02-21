from build import *


class PythonSetuptools(PythonRecipe):
    description = "Easily download, build, install, upgrade, and uninstall Python packages"
    license = "Apache"

    def requirements(self):
        self.requires(f"python/[~{self.settings.python}]")

    def source(self):
        self.get(f"https://github.com/pypa/setuptools/archive/v{self.version}.tar.gz")

    def build(self):
        # Install without python-pip package
        self.exe("python -m ensurepip --default-pip")
        self.exe(f"python -m pip install -Iv --prefix={self.package_folder} setuptools=={self.version}")
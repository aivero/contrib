from build import *


class Npm(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = "A package manager for javascript"
    license = "MIT"
    requires = ("nodejs/[>=16.6.1]",)

    def source(self):
        self.get(f"https://github.com/npm/cli/archive/v{self.version}.tar.gz")

    def build(self):
        os.makedirs(os.path.join(self.package_folder, "lib"))
        self.exe(f'node bin/npm-cli.js install -g --prefix "{self.package_folder}" npm')

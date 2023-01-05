from build import *


class GitLFS(Recipe):
    settings = Recipe.settings + ("compiler",)
    description = " Git extension for versioning large files "
    license = "MIT"
    build_requires = (
        "make/[^4.3]",
        "go/[^1.15.3]",
    )

    def source(self):
        self.get(f"https://github.com/git-lfs/git-lfs/archive/refs/tags/v{self.version}.tar.gz")

    def build(self):
        args = [
            f"prefix={self.package_folder}",
            f"GIT_LFS_SHA={self.version}",
            f"VERSION={self.version}",
        ]
        self.make(args, target=f"bin/git-lfs")

    def package(self):
        self.copy("*lfs*", src=f"git-lfs-{self.version}.src/bin", dst="bin")

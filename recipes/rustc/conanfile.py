from build import *


class Rustc(Recipe):
    description = "Systems programming language focused on safety, speed and concurrency"
    license = "MIT"
    exports = ("*-boostrap-respect-cxxflags.patch",)
    build_requires = (
        "cmake/[^3.18.4]",
        "curl/[^7.72.0]",
        "pkgconf/[^1.7.3]",
        "git/[^2.29.1]",
        "python/[^3]",
    )
    requires = (
        "cc/[^1.0.0]",
        "zlib/[^1.2.11]",
        # Remove when openssl 3.0 is supported
        "openssl1/[>=1.1.1h]",
        "libssl/[^1.0.0]",
    )

    def source(self):
        self.get(f"https://static.rust-lang.org/dist/rustc-{self.version}-src.tar.gz")
        self.patch(f"{self.version}-boostrap-respect-cxxflags.patch")

    def build(self):
        os.environ["RUSTFLAGS"] = "-g -Clinker-plugin-lto -Copt-level=2"
        arch = {"x86_64": "x86_64", "armv8": "aarch64"}[str(self.settings.arch)]
        triple = f"{arch}-unknown-linux-gnu"
        args = [
            f"--host={triple}",
            f"--target={triple}",
            f'--prefix="{self.package_folder}"',
            f'--sysconfdir="{self.package_folder}/etc"',
            f"--llvm-root={self.deps_cpp_info['llvm'].rootpath}",
            "--release-channel=stable",
            "--set=llvm.thin-lto=true",
            "--tools=src,cargo,rustfmt,clippy,rust-analyzer",
            "--enable-option-checking",
            "--enable-llvm-link-shared",
            "--enable-locked-deps",
            "--enable-extended",
            "--enable-vendor",
            "--disable-docs",
            "--disable-compiler-docs",
        ]
        self.exe("./configure", args)
        self.exe("python x.py install")

    def package_info(self):
        self.env_info.RUSTFLAGS = "-g -Copt-level=2"

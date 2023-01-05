from build import *


class Openssl(Recipe):
    description = "TLS/SSL and crypto library"
    license = "BSD"
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "perl/[^5.30.0]",
    )

    def source(self):
        if self.version.startswith("1"):
            self.get(
                f"https://github.com/openssl/openssl/archive/OpenSSL_{self.version.replace('.', '_')}.tar.gz"
            )
        else:
            self.get(f"https://github.com/openssl/openssl/archive/openssl-{self.version}.tar.gz")

    def build(self):
        args = [
            f"--prefix={self.package_folder}",
            "no-ssl3-method",
            "--libdir=lib",
        ]
        if self.options.shared:
            args.append("shared")
        if self.settings.arch == "x86_64":
            args += ["linux-x86_64", "enable-ec_nistp_64_gcc_128"]
        elif self.settings.arch == "armv8":
            args += ["linux-aarch64", "no-afalgeng"]
        self.exe("./Configure", args)
        self.make()

        if not self.options.shared:
            libs = ["crypto", "ssl"]
            for lib in libs:
                os.remove(os.path.join(self.package_folder, "lib", f"lib{lib}.a"))

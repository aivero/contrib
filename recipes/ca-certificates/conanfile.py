from build import *


class CaCertificates(Recipe):
    description = "Common CA certificates PEM files from Mozilla"
    license = "MPL"
    options = {}
    default_options = {}
    build_requires = (
        "cc/[^1.0.0]",
        "make/[^4.3]",
        "openssl/[^3.0.0-alpha7]",
    )

    def source(self):
        self.get(
            f"https://gitlab.alpinelinux.org/alpine/ca-certificates/-/archive/{self.version}/ca-certificates-{self.version}.tar.bz2"
        )

    def build(self):
        os.environ["DESTDIR"] = self.package_folder
        os.environ[
            "CFLAGS"
        ] += f" -ldl -lpthread -I{self.deps_cpp_info['openssl'].rootpath}/include"
        self.make(target="all")

        # NetLock Arany contains invalid utf-8 characters, which is not supported in Python 3.6
        self.exe("mv NetLock_Arany_*.crt NetLock_Arany.crt")
        # Combine certs to ca-certificates.crt
        with open("ca-certificates.crt", "w") as ca_cert:
            for cert in glob.glob(f"{self.src}/*.crt"):
                ca_cert.write(pathlib.Path(cert).read_text())
        os.symlink("ca-certificates.crt", "cert.pem")

    def package(self):
        self.copy("*.crt", dst="share/ca-certificates", keep_path=False)
        self.copy("*ca-certificates.crt", dst="etc/ssl/certs")
        self.copy("*cert.pem", dst="etc/ssl/certs", symlinks=True)

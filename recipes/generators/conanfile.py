from build import *
from conans.model import Generator


class env(Generator):
    @property
    def filename(self):
        return "env.sh"

    @property
    def content(self):
        # Set environment from env_info
        content = 'export PKG_CONFIG_PATH="{}"\n'.format(self.output_path)
        for var, val in self.conanfile.env.items():
            if isinstance(val, str):
                val = [val]
            if len(val) > 1 or (var in os.environ and os.pathsep in os.environ[var]):
                content += 'export {0}={1}:"${{{0}:+:${0}}}"\n'.format(
                    var, os.pathsep.join('"%s"' % p for p in val)
                )
            else:
                content += "export {0}={1}\n".format(var, '"%s"' % val[0])

        return content


class direnv(Generator):
    @property
    def filename(self):
        return ".envrc"

    @property
    def content(self):
        # Set environment from env_info
        content = ""
        for var, val in self.conanfile.env.items():
            if isinstance(val, str):
                val = [val]
            if len(val) > 1 or (var in os.environ and os.pathsep in os.environ[var]):
                content += 'export {0}={1}:"${{{0}:+:${0}}}"\n'.format(
                    var, os.pathsep.join('"%s"' % p for p in val)
                )
            else:
                content += "export {0}={1}\n".format(var, '"%s"' % val[0])

        return content


class gdb(Generator):
    def __init__(self, conanfile):
        super().__init__(conanfile)

    @property
    def filename(self):
        pass

    @property
    def content(self):
        content = {".gdbinit": ""}
        if not "SOURCE_MAP" in self.conanfile.env:
            return {}
        for map in self.conanfile.env["SOURCE_MAP"]:
            if not "|" in map:
                continue
            content[".gdbinit"] += "set substitute-path %s %s\n" % tuple(map.split("|"))

        return content


class lldb(Generator):
    def __init__(self, conanfile):
        super().__init__(conanfile)

    @property
    def filename(self):
        pass

    @property
    def content(self):
        content = {".lldbinit": ""}
        if not "SOURCE_MAP" in self.conanfile.env:
            return {}
        for map in self.conanfile.env["SOURCE_MAP"]:
            if not "|" in map:
                continue
            content[".lldbinit"] += "settings set target.source-map %s %s\n" % tuple(map.split("|"))

        return content


class tools(Generator):
    @property
    def filename(self):
        pass

    @property
    def content(self):
        if not os.path.isdir(self.output_path):
            os.makedirs(self.output_path)

        # Generate wrapper bins
        env_vars = ""
        for var, val in self.conanfile.env.items():
            if isinstance(val, str):
                val = [val]
            if len(val) > 1 or (var in os.environ and os.pathsep in os.environ[var]):
                env_vars += 'export {0}={1}"${{{0}:+:${0}}}"\n'.format(
                    var, os.pathsep.join('"%s"' % p for p in val)
                )
            else:
                env_vars += "export {0}={1}\n".format(var, '"%s"' % val[0])

        # Find rootpath
        # 'dependencies' is not indexable
        for _, cpp_info in self.deps_build_info.dependencies:
            rootpath = cpp_info.rootpath
            break

        # Generate executable wrappers
        bin_path = os.path.join(rootpath, "bin")
        if not os.path.isdir(bin_path):
            return ""
        for exe_name in os.listdir(bin_path):
            exe_path = os.path.join(bin_path, exe_name)
            exe_out_path = os.path.join(self.output_path, exe_name)
            with open(exe_out_path, "w") as exe:
                exe.write("#!/usr/bin/env sh\n")
                exe.write(env_vars)
                exe.write('exec %s "$@"' % exe_path)
            os.chmod(exe_out_path, 0o775)

        return {}


def replace_prefix_in_pc_file(pc_file, prefix):
    with open(pc_file) as f:
        old_prefix = ""
        # Get old prefix
        for l in f:
            if l == "prefix=":
                return f.read().replace("prefix=", "prefix=%s".format(prefix))
            if "prefix=" in l:
                old_prefix = l.split("=")[1][:-1]
                break
        f.seek(0)
        if not old_prefix:
            for l in f:
                if "libdir=" in l:
                    old_prefix = l.split("=")[1][:-5]
                    break
                if "includedir=" in l:
                    old_prefix = l.split("=")[1][:-9]
                    break
        if not old_prefix:
            raise Exception("Could not find package prefix in '%s'" % pc_file)
        f.seek(0)
        return f.read().replace(old_prefix, prefix)


class pkgconf(Generator):
    @property
    def filename(self):
        pass

    @property
    def content(self):
        files = {}

        # Generate pc files
        for _, cpp_info in self.deps_build_info.dependencies:
            pc_paths = [
                os.path.join(cpp_info.rootpath, "lib", "pkgconfig"),
                os.path.join(cpp_info.rootpath, "share", "pkgconfig"),
            ]
            for pc_path in pc_paths:
                if not os.path.isdir(pc_path):
                    continue
                for pc in os.listdir(pc_path):
                    files[pc] = replace_prefix_in_pc_file(
                        os.path.join(pc_path, pc), cpp_info.rootpath
                    )

        # Generate pc files from PKG_CONFIG_SYSTEM_PATH
        if hasattr(self.conanfile, "system_pcs") and "PKG_CONFIG_SYSTEM_PATH" in os.environ:
            if isinstance(self.conanfile.system_pcs, str):
                self.conanfile.system_pcs = set([self.conanfile.system_pcs])
            system_pcs = set(self.conanfile.system_pcs)
            for pc_path in os.environ["PKG_CONFIG_SYSTEM_PATH"].split(os.pathsep):
                for pc in os.listdir(pc_path):
                    pc_name = os.path.splitext(pc)[0]
                    if not pc_name in self.conanfile.system_pcs:
                        continue
                    system_pcs.remove(pc_name)
                    with open(os.path.join(pc_path, pc), "r") as pc_file:
                        files[pc] = pc_file.read()
            if len(system_pcs):
                raise Exception(
                    "'%s' not available in system pkg-config directories" % ", ".join(system_pcs)
                )

        return files


class GeneratorsPackage(Recipe):
    description = "Conan generators"
    license = "MIT"
    requires = ()
    settings = None
    options = None
    default_options = None

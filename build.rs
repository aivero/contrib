fn main() {
    pkg_config::Config::new()
        .atleast_version("0.2.0")
        .probe("depth-meta")
        .unwrap();
}

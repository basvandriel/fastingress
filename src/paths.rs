use std::path::PathBuf;

pub fn get_project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn get_kubernetes_path() -> PathBuf {
    let mut base = get_project_root();
    base.push("kubernetes");

    base
}

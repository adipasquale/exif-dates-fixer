use std::path::PathBuf;

pub fn seeds_pathbuf(name: &str) -> std::path::PathBuf {
    let mut pathbuf: PathBuf = std::env::current_dir().unwrap();
    pathbuf.push("tests-assets");
    pathbuf.push(name);
    pathbuf
}

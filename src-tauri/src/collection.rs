// mod file_infos;
pub use crate::file_infos::FileInfo;

use glob::glob;
use std::path::{Path, PathBuf};

pub struct Collection {
    pub path: PathBuf,
    pub file_infos: Vec<FileInfo>,
}

impl Collection {
    pub fn new(path: &PathBuf) -> Collection {
        let paths = get_paths(path);

        Collection {
            path: path.clone(),
            file_infos: get_fileinfos(paths),
        }
    }
}

fn get_paths(dirpath: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = glob(dirpath.join("**/*.jpg").to_str().unwrap())
        .unwrap()
        .map(|res| res.unwrap())
        .collect();

    paths.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    paths
}

fn get_fileinfos(paths: Vec<PathBuf>) -> Vec<FileInfo> {
    let mut fileinfos: Vec<FileInfo> = paths
        .iter()
        .map(|path: &PathBuf| FileInfo::new_from_path(path))
        .collect();
    fileinfos.sort_by(|a, b| b.date.cmp(&a.date));
    fileinfos
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    fn seeds_pathbuf(name: &str) -> std::path::PathBuf {
        let mut pathbuf: PathBuf = std::env::current_dir().unwrap();
        pathbuf.push("tests-assets");
        pathbuf.push(name);
        pathbuf
    }

    #[test]
    fn test_get_paths() {
        let paths = get_paths(seeds_pathbuf("photos1").as_path());
        assert_eq!(paths.len(), 4);
        assert_eq!(
            paths[0].file_name(),
            Some(OsStr::new("photo_without_date.jpg"))
        );
    }

    #[test]
    fn test_get_paths_empty() {
        let paths = get_paths(seeds_pathbuf("empty").as_path());
        assert_eq!(paths.len(), 0);
    }

    #[test]
    fn test_get_paths_extra_files() {
        let paths = get_paths(seeds_pathbuf("extra-files").as_path());
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_get_paths_nested_folders() {
        let paths = get_paths(seeds_pathbuf("photos-nested").as_path());
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_path_to_fileinfo() {
        let fileinfo = FileInfo::new_from_path(&PathBuf::from(
            "tests-assets/photos1/photo_2017_02_22_a.jpg",
        ));
        assert_eq!(
            fileinfo.path,
            PathBuf::from("tests-assets/photos1/photo_2017_02_22_a.jpg")
        );
        assert_eq!(fileinfo.date, chrono::NaiveDate::from_ymd_opt(2017, 2, 22));
    }

    #[test]
    fn test_get_fileinfos() {
        let paths: Vec<PathBuf> = vec![
            PathBuf::from("tests-assets/photos1/photo_2017_02_22_a.jpg"),
            PathBuf::from("tests-assets/photos1/photo_2018_03_23_a.jpg"),
            PathBuf::from("tests-assets/photos1/photo_2018_03_24_a.jpg"),
        ];
        let fileinfos = get_fileinfos(paths);
        assert_eq!(fileinfos.len(), 3);
        assert_eq!(
            fileinfos[0].path,
            PathBuf::from("tests-assets/photos1/photo_2018_03_24_a.jpg")
        );
        assert_eq!(
            fileinfos[0].date,
            chrono::NaiveDate::from_ymd_opt(2018, 3, 24)
        );
        assert_eq!(
            fileinfos[1].path,
            PathBuf::from("tests-assets/photos1/photo_2018_03_23_a.jpg")
        );
        assert_eq!(
            fileinfos[1].date,
            chrono::NaiveDate::from_ymd_opt(2018, 3, 23)
        );
        assert_eq!(
            fileinfos[2].path,
            PathBuf::from("tests-assets/photos1/photo_2017_02_22_a.jpg")
        );
        assert_eq!(
            fileinfos[2].date,
            chrono::NaiveDate::from_ymd_opt(2017, 2, 22)
        );
    }
}

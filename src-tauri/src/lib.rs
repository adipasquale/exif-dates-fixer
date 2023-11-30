mod collection;
mod file_infos;

pub use collection::Collection;
pub use file_infos::FileInfo;

use chrono::NaiveDate;
use rexiv2::Rexiv2Error;
use std::path::PathBuf;

pub fn set_date(path: &PathBuf, date: &NaiveDate) -> Result<FileInfo, Rexiv2Error> {
    rexiv2::initialize().expect("Unable to initialize rexiv2");
    rexiv2::set_log_level(rexiv2::LogLevel::ERROR);
    let mut file_info = FileInfo::new_from_path(path);
    file_info
        .metadata
        .set_tag_string(
            "Exif.Photo.DateTimeOriginal",
            &date.format("%Y:%m:%d 00:00:00").to_string(),
        )
        .expect("Unable to set date");

    file_info
        .metadata
        .set_tag_string("Exif.Photo.UserComment", "fixed-with-exif-dates-fixer")?;

    file_info.metadata.save_to_file(path)?;
    file_info.date = Some(date.clone());

    Ok(file_info)
}

#[cfg(test)]
mod test_helpers;

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    fn setup() {
        let path = test_helpers::seeds_pathbuf("photos1/photo_2018_03_24_a.jpg");
        let tmp_path = test_helpers::seeds_pathbuf("photo_tmp.jpg");
        std::fs::copy(path, tmp_path).expect("Unable to copy file");
    }

    fn teardown() {
        let tmp_path = test_helpers::seeds_pathbuf("photo_tmp.jpg");
        std::fs::remove_file(tmp_path).expect("Unable to remove file");
    }

    // from https://medium.com/all-about-rust/example-for-setup-and-teardown-in-testing-using-rust-3606bca60a31
    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        setup();
        let result = panic::catch_unwind(|| test());
        teardown();
        assert!(result.is_ok())
    }

    #[test]
    fn test_set_date() {
        run_test(|| {
            let path = test_helpers::seeds_pathbuf("photo_tmp.jpg");
            let date = NaiveDate::from_ymd_opt(2020, 5, 21).unwrap();
            let res = set_date(&path, &date);
            assert!(res.is_ok());
            let file_info_res = res.unwrap();
            assert_eq!(file_info_res.date, Some(date));
            assert_eq!(
                file_info_res
                    .metadata
                    .get_tag_string("Exif.Photo.DateTimeOriginal")
                    .unwrap(),
                "2020:05:21 00:00:00"
            );
            assert_eq!(
                file_info_res
                    .metadata
                    .get_tag_string("Exif.Photo.UserComment")
                    .unwrap(),
                "fixed-with-exif-dates-fixer"
            );
            let metadata = rexiv2::Metadata::new_from_path(path).unwrap();
            assert_eq!(
                metadata
                    .get_tag_string("Exif.Photo.DateTimeOriginal")
                    .unwrap(),
                "2020:05:21 00:00:00"
            );
            assert_eq!(
                metadata.get_tag_string("Exif.Photo.UserComment").unwrap(),
                "fixed-with-exif-dates-fixer"
            );
        })
    }
}

use chrono::Datelike;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub metadata: rexiv2::Metadata,
    pub filename: String,
    pub date: Option<chrono::NaiveDate>,
    pub exif_date_tags: HashMap<String, String>,
}

impl FileInfo {
    pub fn new_from_path(path: &PathBuf) -> FileInfo {
        let metadata = rexiv2::Metadata::new_from_path(&path).ok();
        let date = if let Some(md) = &metadata {
            parse_date(&md)
        } else {
            None
        };

        let exif_date_tags = if let Some(metadata) = &metadata {
            extract_exif_date_tags(&metadata)
        } else {
            HashMap::new()
        };

        FileInfo {
            path: path.clone(),
            filename: path.file_name().unwrap().to_str().unwrap().to_string(),
            date,
            exif_date_tags,
            metadata: metadata.unwrap(),
        }
    }
}

impl Serialize for FileInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FileInfo", 3)?;
        s.serialize_field("path", &self.path)?;
        s.serialize_field("filename", &self.filename)?;
        s.serialize_field("exifDateTags", &self.exif_date_tags)?;
        if let Some(date) = self.date {
            s.serialize_field("date", &date.format("%Y-%m-%d").to_string())?;
            s.serialize_field("year", &date.year())?;
            s.serialize_field("week", &date.iso_week().week())?;
        } else {
            s.serialize_field("date", &"")?;
            s.serialize_field("year", &0)?;
            s.serialize_field("week", &0)?;
        }
        s.end()
    }
}

fn extract_exif_date_tags(metadata: &rexiv2::Metadata) -> HashMap<String, String> {
    let mut exif_date_tags = HashMap::new();
    if let Some(exif_tags_keys) = metadata.get_exif_tags().ok() {
        for key in exif_tags_keys {
            if key.to_ascii_lowercase().contains("date") {
                if let Some(value) = metadata.get_tag_string(&key).ok() {
                    exif_date_tags.insert(key, value);
                }
            }
        }
    }
    exif_date_tags
}

fn parse_date(metadata: &rexiv2::Metadata) -> Option<chrono::NaiveDate> {
    let raw_date = metadata
        .get_tag_string("Exif.Photo.DateTimeOriginal")
        .ok()?;

    if raw_date == "0000:00:00 00:00:00" {
        return None;
    }
    match chrono::NaiveDate::parse_from_str(&raw_date, "%Y:%m:%d %H:%M:%S") {
        Ok(date) => Some(date),
        Err(_) => {
            println!("Unable to parse date: {}", raw_date);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers;
    use std::path::PathBuf;

    #[test]
    fn test_extract_exif_metadata() {
        let metadata = rexiv2::Metadata::new_from_path(&PathBuf::from(
            "tests-assets/photos1/photo_2017_02_22_a.jpg",
        ));
        // it should not panic
        assert!(metadata.is_ok());
    }

    #[test]
    fn test_parse_date() {
        let metadata = rexiv2::Metadata::new_from_path(&PathBuf::from(
            "tests-assets/photos1/photo_2017_02_22_a.jpg",
        ));
        let date = parse_date(&metadata.unwrap());
        assert_eq!(date, chrono::NaiveDate::from_ymd_opt(2017, 2, 22));
    }

    #[test]
    fn test_parse_date_missing() {
        let metadata = rexiv2::Metadata::new_from_path(&PathBuf::from(
            "tests-assets/photos1/photo_without_date.jpg",
        ));
        let date = parse_date(&metadata.unwrap());
        assert_eq!(date, None)
    }

    #[test]
    fn test_extract_exif_date_tags() {
        let metadata: rexiv2::Metadata = rexiv2::Metadata::new_from_path(
            test_helpers::seeds_pathbuf("photos1/photo_2018_03_24_a.jpg"),
        )
        .unwrap();

        let res = extract_exif_date_tags(&metadata);
        assert_eq!(res.len(), 3);
        assert_eq!(
            res.get("Exif.Image.DateTime").unwrap(),
            "2018:04:25 01:01:01"
        );
        assert_eq!(
            res.get("Exif.Photo.DateTimeDigitized").unwrap(),
            "2018:03:24 00:00:00"
        );
        assert_eq!(
            res.get("Exif.Photo.DateTimeOriginal").unwrap(),
            "2018:03:24 00:00:00"
        );
    }

    #[test]
    fn test_serialize_fileinfo() {
        let mut date_tags = HashMap::new();
        date_tags.insert(
            "Exif.Photo.DateTimeOriginal".to_string(),
            "2017:02:22 00:00:00".to_string(),
        );
        date_tags.insert(
            "Exif.Photo.DateTime".to_string(),
            "2018:04:25 01:01:01".to_string(),
        );
        let fileinfo = FileInfo {
            path: PathBuf::from("/Photos/tests-assets/photos1/photo_2017_02_22_a.jpg"),
            date: chrono::NaiveDate::from_ymd_opt(2017, 2, 22),
            exif_date_tags: date_tags,
            filename: "photo_2017_02_22_a.jpg".to_string(),
            metadata: rexiv2::Metadata::new_from_path(test_helpers::seeds_pathbuf(
                "photos1/photo_2017_02_22_a.jpg",
            ))
            .unwrap(),
        };
        let serialized = serde_json::to_string(&fileinfo).unwrap();
        let unserialized: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            unserialized["path"],
            "/Photos/tests-assets/photos1/photo_2017_02_22_a.jpg"
        );
        assert_eq!(unserialized["date"], "2017-02-22");
        assert_eq!(unserialized["year"], 2017);
        assert_eq!(unserialized["week"], 8);
        assert_eq!(
            unserialized["exifDateTags"]["Exif.Photo.DateTimeOriginal"],
            "2017:02:22 00:00:00"
        );
        assert_eq!(
            unserialized["exifDateTags"]["Exif.Photo.DateTime"],
            "2018:04:25 01:01:01"
        );
        assert_eq!(unserialized["filename"], "photo_2017_02_22_a.jpg")
    }
}

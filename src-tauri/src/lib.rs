mod file_infos;
pub use file_infos::FileInfo;

mod collection;
pub use collection::Collection;

// pub fn list(dirpath: &str) -> Vec<FileInfo> {
//     rexiv2::initialize().expect("Unable to initialize rexiv2");
//     rexiv2::set_log_level(rexiv2::LogLevel::ERROR);
//     call collection.new
// }

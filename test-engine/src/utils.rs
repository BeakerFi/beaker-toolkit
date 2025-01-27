pub use std::path::PathBuf;
pub use std::{path::Path, time::SystemTime};
pub use walkdir::WalkDir;

pub fn get_last_modified_time(path: &Path) -> Option<SystemTime> {
    let mut latest_mod_time = SystemTime::UNIX_EPOCH;

    for entry in WalkDir::new(path) {
        let entry = entry.ok()?;
        let metadata = entry.metadata().ok()?;
        let modified_time = metadata.modified().ok().unwrap();

        if modified_time > latest_mod_time {
            latest_mod_time = modified_time;
        }
    }

    Some(latest_mod_time)
}

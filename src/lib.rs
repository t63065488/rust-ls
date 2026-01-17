use std::{fs, path::Path};

pub fn get_files(dir: &Path, files_only: bool) -> Vec<String> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory {:?}: {}", dir, e);
            return Vec::new();
        }
    };
    entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            if files_only {
                return entry.path().is_file();
            } else {
                return true;
            }
        })
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect()
}

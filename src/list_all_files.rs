use crate::types::File;
use std::time::SystemTime;
use walkdir::WalkDir;

/// Retrieves all files under the given path.
pub fn execute(path: &str) -> Vec<File> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .map(|f| {
            let name = String::from(f.file_name().to_string_lossy());
            let extension = f
                .path()
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string();
            let path = f.path().clone();
            let directory = match f.path().parent() {
                Some(p) => p.to_string_lossy().to_string(),
                None => "".into(),
            };

            let modified_at = f
                .metadata()
                .unwrap()
                .modified()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();

            let created_at = f
                .metadata()
                .unwrap()
                .created()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();

            File {
                created_at,
                directory,
                name,
                path: path.to_string_lossy().to_string(),
                extension,
                modified_at,
            }
        })
        .collect()
}

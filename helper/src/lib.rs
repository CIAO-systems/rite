use std::{env, path::Path};

/// INFO logs the current working directoy 
pub fn pwd() {
    if let Ok(cwd) = env::current_dir() {
        log::info!("{}", cwd.display());
    }
}

/// Returns the full absolute path of the `filename`
/// 
/// If the filename has no path, the current working directory is added before the 
/// filename
/// 
pub fn get_full_path(filename: &str) -> std::io::Result<std::path::PathBuf> {
    let path = Path::new(filename);
    // If the path is already absolute, return it
    if path.is_absolute() {
        return Ok(normalize_path(path));
    }

    // Get the current directory and append the relative filename
    let mut full_path = std::env::current_dir()?;
    full_path.push(path);

    Ok(normalize_path(&full_path))
}

/// Converts a path to normalized form (eliminating . and .. relative path components)
/// 
/// For example: 
/// - /home/user/../user/data/./filename.txt ->
///   /home/user/data/filename.txt
/// 
pub fn normalize_path(path: &Path) -> std::path::PathBuf {
    let mut normalized = std::path::PathBuf::new();

    for component in path.components() {
        match component {
            // Skip current directory components
            std::path::Component::CurDir => continue,
            // Go up one directory when encountering parent directory
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            // Add other components (root, normal path segments)
            _ => normalized.push(component),
        }
    }

    normalized
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::get_full_path;

    #[test]
    fn test_normalize() -> Result<(), Box<dyn std::error::Error>> {
        let cwd = env::current_dir().unwrap_or("".into());

        let absolute_path = get_full_path("/home/user/documents/file.txt")?;
        assert_eq!(
            "/home/user/documents/file.txt",
            absolute_path.to_str().unwrap()
        );

        let absolute_path = get_full_path("/home/user/../documents/file.txt")?;
        assert_eq!("/home/documents/file.txt", absolute_path.to_str().unwrap());

        let absolute_path = get_full_path("example.txt")?;
        assert_eq!(
            format!("{}/example.txt", cwd.display()),
            absolute_path.to_str().unwrap()
        );

        let absolute_path = get_full_path("../example.txt")?;
        let cwdp = cwd.parent().unwrap_or(&cwd);
        assert_eq!(
            format!("{}/example.txt", cwdp.display()),
            absolute_path.to_str().unwrap()
        );

        Ok(())
    }
}

use std::io::BufRead;

/// Represents a single file and its related information
#[derive(Debug)]
pub struct File {
    path: std::path::PathBuf,
    lines: usize,
    language: Option<String>,
}

impl File {
    /// Parse a [`File`] from the given [`path`][std::path::Path]
    pub fn from_path(path: &std::path::Path) -> std::io::Result<File> {
        // Create a buffered reader to read the file-contents
        let file = std::fs::File::open(&path)?;
        let reader = std::io::BufReader::new(file);

        // Count the number of lines in the file
        let lines = reader.lines().count();

        // Try to determine the language from the file extension
        let language = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase());

        Ok(File {
            path: path.to_path_buf(),
            lines,
            language,
        })
    }
}

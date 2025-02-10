use std::io::BufRead;

/// Represents a single file and its related information
#[derive(Debug)]
pub struct File {
    path: std::path::PathBuf,
    lines: usize,
}

impl File {
    /// Parse a [`File`] from the given [`path`][std::path::Path]
    pub fn from_path(path: &std::path::Path) -> std::io::Result<File> {
        // Create a buffered reader to read the file-contents
        let file = std::fs::File::open(&path)?;
        let reader = std::io::BufReader::new(file);

        // Count the number of lines in the file
        let lines = reader.lines().count();

        Ok(File {
            path: path.to_path_buf(),
            lines,
        })
    }
}

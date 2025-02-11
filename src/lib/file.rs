use std::io::BufRead;

use crate::language::Language;

/// Represents a single file and its related information
#[derive(Debug)]
pub struct File {
    pub path: std::path::PathBuf,
    pub lines: usize,
    pub language: Language,
}

impl File {
    /// Parse a [`File`] from the given [`path`][std::path::Path]
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<File> {
        // Create a buffered reader to read the file-contents
        let file = std::fs::File::open(&path)?;
        let reader = std::io::BufReader::new(file);

        // Count the number of lines in the file
        let mut lines = 0;
        for line in reader.lines() {
            if line.is_ok() {
                lines += 1;
            }
        }

        // Try to determine the language from the file extension
        let path = path.as_ref().to_path_buf();
        let language = Language::from_path(&path);

        Ok(File {
            path,
            lines,
            language,
        })
    }
}

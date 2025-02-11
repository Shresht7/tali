use std::io::BufRead;

use crate::language::Language;

#[derive(Default)]
pub struct FileScanner {
    lines: bool,
    bytes: bool,
}

impl FileScanner {
    pub fn new(lines: bool, bytes: bool) -> FileScanner {
        FileScanner { lines, bytes }
    }

    pub fn scan<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<FileScanResult> {
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

        Ok(FileScanResult {
            path,
            lines,
            language,
        })
    }
}

/// Represents a single file and its related information
#[derive(Debug)]
pub struct FileScanResult {
    pub path: std::path::PathBuf,
    pub lines: usize,
    // pub bytes: u64,
    pub language: Language,
}

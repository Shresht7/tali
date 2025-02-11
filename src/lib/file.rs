use std::io::BufRead;

use crate::language::Language;

/// Represents a single file and it's information
#[derive(Debug)]
pub struct File {
    pub path: std::path::PathBuf,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: usize,
    pub language: Language,
}

impl File {
    /// Opens and scans the file at the given path, counting lines, words, characters, and bytes.
    pub fn scan<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<File> {
        // Open file
        let file = std::fs::File::open(&path)?;

        // Retrieve the number of bytes from the file-metadata
        let bytes = file.metadata()?.len() as usize;

        // Create a buffered reader
        let reader = std::io::BufReader::new(file);

        // Setup the counters
        let mut lines = 0;
        let mut words = 0;
        let mut chars = 0;

        // Process each line...
        for line in reader.lines() {
            let line = line?; // Propagate error up, if any
            lines += 1; // Increment the line count
            words += line.split_whitespace().count(); // Increment the word count
            chars += line.chars().count(); // Increment the characters count
        }

        // Determine the language from the file extension
        let path = path.as_ref().to_path_buf();
        let language = Language::from_path(&path);

        Ok(File {
            path,
            lines,
            words,
            chars,
            bytes,
            language,
        })
    }
}

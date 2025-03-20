use serde::Serialize;

use std::io::BufRead;

use crate::helpers::language::Language;

/// Represents a scanned file and its computed metrics.
///
/// The metrics include:
/// - `lines`: The number of lines (as determined by [`BufRead::lines`]).
/// - `words`: The number of words (splitting each line on whitespace).
/// - `chars`: The total number of Unicode characters (excluding newline characters).
/// - `bytes`: The total number of bytes (queried from file [`metadata`][std::fs::Metadata]).
/// - `language`: The [`language`][Language] detected from the file extension.
#[derive(Debug, Clone, Serialize)]
pub struct File {
    /// The path to the file
    pub path: std::path::PathBuf,
    /// The number of lines in the file
    pub lines: usize,
    /// The number of words in the file
    pub words: usize,
    /// The number of unicode characters in the file
    pub chars: usize,
    /// The size of the file in bytes
    pub bytes: u64,
    /// The language used in this file
    pub language: Language,
    /// The number of files this entry represents. Usually 1 unless aggregated under a language.
    pub count: usize,
}

impl File {
    /// Scans the file at the given [`path`][std::path::Path] and computes various [metrics][File].
    ///
    /// This function opens the file, reads it once line by line,
    /// and computes the number of `lines`, `words`, and `characters`. The `byte count` is obtained
    /// from the file [`metadata`][std::fs::Metadata]. The file’s [`language`][Language] is determined from its extension.
    ///
    /// # Arguments
    ///
    /// * `path` - A reference to a path of the file to scan.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if opening the file, reading from it, or obtaining its metadata fails.
    pub fn scan<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<File> {
        // Open file
        let file = std::fs::File::open(&path)?;

        // Retrieve the number of bytes from the file-metadata
        let bytes = file.metadata()?.len();

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
            count: 1,
        })
    }

    /// Scans the given [reader][BufRead]  and computes various [metrics][File]
    ///
    /// The function reads the reader line-by-line, and computes the number of `lines`, `words`, `characters` and `bytes`.
    /// The language, currently, is always assumed to be [Text][Language::Text].
    ///
    /// # Arguments
    ///
    /// * `reader` - A [BufReader][BufRead] to the contents to read
    ///
    /// # Errors
    ///
    /// Returns an error if we fail to [read][BufRead::lines] a line from the [reader][BufRead]
    pub fn scan_reader<R: BufRead>(reader: R) -> std::io::Result<File> {
        // Setup the counter
        let mut lines = 0;
        let mut words = 0;
        let mut chars = 0;
        let mut bytes = 0;

        // Process each line...
        for line in reader.lines() {
            let line = line?; // Propagate error up, if any
            lines += 1; // Increment the line count
            words += line.split_whitespace().count(); // Increment the word count
            chars += line.chars().count(); // Increment the characters count
            bytes += line.len() as u64; // Increment the number of bytes by adding the length
        }
        bytes += (lines as u64) - 1; // Adjust the number of bytes to account for the missing \n (consumed by reader.lines()) and remove trailing \n

        let path = std::path::Path::new("STDIN").to_path_buf();
        let language = Language::Text; // Default to plain-text for now.

        Ok(File {
            path,
            lines,
            words,
            chars,
            bytes,
            language,
            count: 1,
        })
    }
}

impl std::ops::Add for File {
    type Output = File;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            path: rhs.path.clone(),
            lines: self.lines + rhs.lines,
            words: self.words + rhs.words,
            chars: self.chars + rhs.words,
            bytes: self.bytes + rhs.bytes,
            language: rhs.language.clone(),
            count: self.count + rhs.count,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    /// Helper to create a temporary file with the given contents
    /// The file is written to the OS temporary directory with a fixed name.
    fn create_temp_file(contents: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push("temp_test_file_scan.txt");
        let mut file = std::fs::File::create(&path).expect("Failed to create temporary file");
        file.write_all(contents.as_bytes())
            .expect("Failed to write to temporary file");
        path
    }

    /// Helper to remove the temporary file
    fn cleanup<P: AsRef<std::path::Path>>(path: P) {
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_scan_file_metrics() {
        // Prepare a sample file.
        // The contents has two lines:
        // "Hello World" and "Rust is awesome"
        // Without the newline at the end of the file, the counts are:
        //  lines: 2
        //  words: 2 + 3 = 5
        //  chars: 11 + 15 = 26
        //  The byte count will equal the length of the file contents in bytes
        let contents = "Hello World\nRust is awesome";
        let temp_path = create_temp_file(contents);

        // Scan the file
        let metrics = File::scan(&temp_path).expect("Failed to scan file");

        // Check the metrics
        assert_eq!(metrics.lines, 2, "Line count mismatch");
        assert_eq!(metrics.words, 5, "Word count mismatch");
        assert_eq!(metrics.chars, 26, "Character count mismatch");
        assert_eq!(metrics.bytes, contents.len() as u64, "Byte count mismatch");

        // Perform cleanup
        cleanup(&temp_path);
    }

    #[test]
    fn test_scan_reader_metrics() {
        // Prepare the reader
        let contents = "We are\nreading this from\na buffered reader";
        let reader = std::io::Cursor::new(contents);

        // Scan the reader
        let metrics = File::scan_reader(reader).expect("Failed to scan reader");

        // Check the metrics
        assert_eq!(metrics.path.to_str(), Some("STDIN"), "Path mismatch");
        assert_eq!(metrics.lines, 3, "Line count mismatch");
        assert_eq!(metrics.words, 8, "Word count mismatch");
        assert_eq!(metrics.chars, 40, "Character count mismatch");
        assert_eq!(metrics.bytes, contents.len() as u64, "Byte count mismatch");
        assert_eq!(metrics.language, Language::Text, "Language mismatch");
    }
}

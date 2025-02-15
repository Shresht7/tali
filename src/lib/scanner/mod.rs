use std::collections::HashMap;

use serde::Serialize;

use crate::helpers::language::Language;

mod file;
pub use file::*;

// -------
// SCANNER
// -------

/// Scan the given [`paths`][std::path::Path] and record [file information][File] such as the line, word, character, and byte counts for each file.
pub fn scan<P: AsRef<std::path::Path>>(paths: &[P]) -> std::io::Result<ScanResults> {
    // A vector to collect the files
    let mut files = Vec::new();

    // Accumulators
    let mut total = Totals::default();
    let mut max = Max::default();

    for path in paths {
        if path.as_ref().is_file() {
            // Parse the file, accumulate stats, and add to the collection
            let file = File::scan(path)?;
            total.add(&file);
            max.track(&file);
            files.push(file);
        } else {
            // Build a directory walker that respects `.gitignore` and other hidden files
            let walker = ignore::WalkBuilder::new(&path).build();

            // Iterate over all the entries
            for result in walker {
                match result {
                    Ok(entry) if entry.path().is_file() => {
                        // Parse the file, accumulate stats, and add it to the collection
                        let file = File::scan(entry.path())?;
                        total.add(&file);
                        max.track(&file);
                        files.push(file);
                    }

                    Ok(_) => {} // Ignore directories and symlinks
                    Err(e) => eprintln!("Error: {}", e), // Report errors
                }
            }
        }
    }

    Ok(ScanResults { files, total, max })
}

// ------------
// SCAN RESULTS
// ------------

/// Represents the aggregate scan results
#[derive(Debug, Serialize)]
pub struct ScanResults {
    /// The collection of all file results containing information like the number of lines, words, chars and bytes
    pub files: Vec<File>,
    /// The aggregate total number of lines, words, chars and bytes
    pub total: Totals,
    /// The max values for the number of lines, words, chars, and bytes across the results
    pub max: Max,
}

impl ScanResults {
    /// Groups the files by language and returns a [`HashMap`]
    fn group_by_language(&self) -> HashMap<Language, Vec<&File>> {
        let mut groups = HashMap::new();

        for file in &self.files {
            groups
                .entry(file.language.clone())
                .or_insert_with(Vec::new)
                .push(file)
        }

        groups
    }
}

// ------------
// ACCUMULATORS
// ------------

/// Represents the accumulated total number of lines, words, chars and bytes in [`ScanResults`]
#[derive(Debug, Default, Serialize)]
pub struct Totals {
    pub files: usize,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: u64,
}

impl Totals {
    /// Add the [`File`] statistics to the totals accumulator
    fn add(&mut self, file: &File) {
        self.files += 1;
        self.lines += file.lines;
        self.words += file.words;
        self.chars += file.chars;
        self.bytes += file.bytes;
    }
}

/// Represents the max values for the number of lines, words, chars and bytes in [`ScanResults`]
#[derive(Debug, Default, Serialize)]
pub struct Max {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: u64,
}

impl Max {
    /// Update the max values for the number of lines, words, chars and bytes by comparing it with a [`File`]
    fn track(&mut self, file: &File) {
        self.lines = self.lines.max(file.lines);
        self.words = self.words.max(file.words);
        self.chars = self.chars.max(file.chars);
        self.bytes = self.bytes.max(file.bytes);
    }
}
